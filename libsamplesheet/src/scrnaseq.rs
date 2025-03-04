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
use tracing::warn;

use color_eyre::eyre::Result;

use crate::prelude::*;

#[derive(Debug, NfCore)]
pub struct ScrnaSeq<'a> {
    input_dir: &'a Path,
    fastq_ext: &'a str,
    expected_cells: &'a i64,
    output_prefix: &'a Option<String>,
}

impl<'a> ScrnaSeq<'a> {
    pub fn new(
        input_dir: &'a Path,
        fastq_ext: &'a str,
        expected_cells: &'a i64,
        output_prefix: &'a Option<String>,
    ) -> Self {
        Self {
            input_dir,
            fastq_ext,
            expected_cells,
            output_prefix,
        }
    }
}

impl<'a> FindInputs for ScrnaSeq<'a> {
    fn find_files(&self) -> Result<Vec<PathBuf>> {
        // define the full pattern
        let pattern = format!("{}/*{}", &self.input_dir.display(), &self.fastq_ext);

        // iterate through entries and make sure they aren't symlinks
        let fastq_paths: Vec<PathBuf> = glob(&pattern)?.flatten().collect();

        Ok(fastq_paths)
    }
}

impl RetrieveSampleIds for ScrnaSeq<'_> {
    fn retrieve_samples(&self, file_paths: &[PathBuf]) -> Result<HashSet<Rc<str>>> {
        let illumina_pattern = Regex::new(r"_(?:S\d+_)?(?:L\d+_)?R\d+(?:_\d+)?\.fastq\.gz$")?;

        let samples = file_paths
            .iter()
            .map(|path| {
                path.file_name()
                    .unwrap_or(OsStr::new(""))
                    .to_string_lossy()
                    .into_owned()
            })
            .map(|x| {
                let id = illumina_pattern.replace_all(&x, "").to_string();
                Rc::from(id)
            })
            .collect();

        Ok(samples)
    }
}

impl ConcatFields for ScrnaSeq<'_> {
    fn concat_lines(
        &self,
        sample_ids: &HashSet<Rc<str>>,
        fastq_paths: &[std::path::PathBuf],
    ) -> Vec<String> {
        sample_ids
            .iter()
            .filter_map(|x| collect_per_sample(x, fastq_paths, self.expected_cells).ok())
            .collect::<Vec<String>>()
    }
}

impl WriteCsvLines for ScrnaSeq<'_> {
    fn write_lines(&self, lines: &[String], header: &str) -> Result<()> {
        let out_name = match self.output_prefix {
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

impl GiveASheet for ScrnaSeq<'_> {
    fn give_a_sheet(self) -> Result<()> {
        let fastq_paths = &self.find_files()?;
        let sample_ids = &self.retrieve_samples(fastq_paths)?;
        check_sample_ids(sample_ids);
        let lines = &self.concat_lines(sample_ids, fastq_paths);
        let header = "sample,fastq_1,fastq_2,expected_cells";
        self.write_lines(lines, header)
    }
}

fn check_sample_ids(sample_ids: &HashSet<Rc<str>>) {
    for id in sample_ids {
        if id.chars().count() >= 64 {
            warn!("Sample id {} is 64 or more characters long, which is maximum enforced by some of the SCRNAseq aligners. SCRNAseq may crash if you don't manually shorten the id in your samplesheet.", id)
        }
    }
}

fn collect_per_sample(
    sample_id: &str,
    fastq_paths: &[PathBuf],
    expected_cells: &i64,
) -> Result<String> {
    // figure out which FASTQ files go with the provided sample_id
    let sample_fastqs: Vec<&str> = fastq_paths
        .iter()
        .map(|x| x.to_str().unwrap_or(""))
        .filter(|x| x.contains(sample_id))
        .collect();

    // pull out the R1 and R2 FASTQ files
    let fastq1 = sample_fastqs
        .iter()
        .find(|x| x.contains("_R1_"))
        .ok_or("No fastq file with 'R1' found")
        .unwrap();
    let fastq2 = sample_fastqs
        .iter()
        .find(|x| x.contains("_R2_"))
        .ok_or("No fastq file with 'R2' found")
        .unwrap();

    let cell_str = expected_cells.to_string();

    // instantiate an illumina line and return it
    Ok([sample_id, fastq1, fastq2, &cell_str].join(","))
}
