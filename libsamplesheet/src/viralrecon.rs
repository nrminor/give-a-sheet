#![allow(clippy::pedantic, clippy::perf)]

use color_eyre::{eyre::eyre, Result};
use glob::glob;
use nfcore_derive::NfCore;
use regex::Regex;
use std::{
    collections::HashSet,
    ffi::OsStr,
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::prelude::*;

#[derive(Debug, NfCore)]
pub struct ViralRecon<'a> {
    input_dir: &'a Path,
    fastq_ext: &'a str,
    platform: &'a SeqPlatform,
    output_prefix: &'a Option<String>,
}

impl<'a> ViralRecon<'a> {
    pub fn new(
        input_dir: &'a Path,
        fastq_ext: &'a str,
        platform: &'a SeqPlatform,
        output_prefix: &'a Option<String>,
    ) -> Self {
        Self {
            input_dir,
            fastq_ext,
            platform,
            output_prefix,
        }
    }
}

impl FindInputs for ViralRecon<'_> {
    fn find_files(&self) -> Result<Vec<PathBuf>> {
        // define the full pattern
        let pattern = format!("{}/*{}", &self.input_dir.display(), &self.fastq_ext);

        // iterate through entries and make sure they aren't symlinks
        let fastq_paths: Vec<PathBuf> = glob(&pattern)?.flatten().collect();

        Ok(fastq_paths)
    }
}

impl ConcatFields for ViralRecon<'_> {
    fn concat_lines(&self, sample_ids: &HashSet<Rc<str>>, fastq_paths: &[PathBuf]) -> Vec<String> {
        sample_ids
            .iter()
            .filter_map(|x| self.platform.collect_by_platform(x, fastq_paths).ok())
            .collect::<Vec<String>>()
    }
}

impl WriteCsvLines for ViralRecon<'_> {
    fn write_lines(&self, lines: &[String], header: &str) -> Result<()> {
        let out_name = match &self.output_prefix {
            Some(prefix) => format!("{}_samplesheet.csv", prefix),
            None => String::from("samplesheet.csv"),
        };

        let file = File::create(out_name)?;
        let mut buf_writer = BufWriter::new(file);

        writeln!(buf_writer, "{}", header)?;
        lines
            .iter()
            .try_for_each(|line| writeln!(buf_writer, "{}", line))?;

        Ok(())
    }
}

impl GiveASheet for ViralRecon<'_> {
    fn give_a_sheet(self) -> Result<()> {
        let fastq_paths = &self.find_files()?;
        let sample_ids = &self.platform.retrieve_samples(fastq_paths)?;
        let lines = &self.concat_lines(sample_ids, fastq_paths);
        let header = "sample,fastq_1,fastq_2";
        self.write_lines(lines, header)
    }
}

impl RetrieveSampleIds for SeqPlatform {
    fn retrieve_samples(&self, file_paths: &[PathBuf]) -> Result<HashSet<Rc<str>>> {
        match self {
            // handle paired end FASTQ files for Illumina
            SeqPlatform::Illumina => {
                let illumina_pattern = Regex::new(r"_L\d{3}_R\d_\d{3}\.fastq\.gz$")?;
                let paired_files = file_paths
                    .iter()
                    .map(|path| {
                        Rc::from(
                            path.file_name()
                                .unwrap_or(OsStr::new(""))
                                .to_string_lossy()
                                .as_ref(),
                        )
                    })
                    .map(|x| Rc::from(illumina_pattern.replace_all(&x, "").to_string()))
                    .collect();
                Ok(paired_files)
            }
            // handle per-barcode single FASTQs for Nanopore
            SeqPlatform::Nanopore => {
                let ont_files = file_paths
                    .iter()
                    .map(|path| {
                        Rc::from(
                            path.file_name()
                                .unwrap_or(OsStr::new(""))
                                .to_string_lossy()
                                .replace(".fastq.gz", "")
                                .as_ref(),
                        )
                    })
                    .collect();
                Ok(ont_files)
            }
        }
    }
}

pub trait CollectByPlatform {
    fn collect_by_platform<'a>(
        &self,
        sample_id: &'a str,
        fastq_paths: &'a [PathBuf],
    ) -> Result<String>;
}

impl CollectByPlatform for SeqPlatform {
    fn collect_by_platform<'a>(
        &self,
        sample_id: &'a str,
        fastq_paths: &'a [PathBuf],
    ) -> Result<String> {
        match self {
            SeqPlatform::Illumina => {
                // figure out which FASTQ files go with the provided sample_id
                let sample_fastqs: Vec<&str> = fastq_paths
                    .iter()
                    .map(|x| x.to_str().unwrap_or(""))
                    .filter(|x| x.contains(sample_id))
                    .collect();

                // pull out the R1 and R2 FASTQ files
                let fastq1 = sample_fastqs
                    .iter()
                    .find(|x| x.contains("R1"))
                    .ok_or(eyre!("No fastq file with 'R1' found"))?;
                let fastq2 = sample_fastqs
                    .iter()
                    .find(|x| x.contains("R2"))
                    .ok_or(eyre!("No fastq file with 'R2' found"))?;

                // instantiate an illumina line and return it
                Ok([sample_id, fastq1, fastq2].join(","))
            }
            SeqPlatform::Nanopore => {
                // pull out the barcode
                let barcode = if sample_id.starts_with('0') {
                    sample_id.replace("barcode", "").chars().skip(1).collect()
                } else {
                    sample_id.replace("barcode", "")
                };

                // instantiate a Nanopore line and return it
                Ok([sample_id, &barcode].join(","))
            }
        }
    }
}
