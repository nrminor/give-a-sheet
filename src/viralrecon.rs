use crate::utils::{write_lines, SeqPlatform};
use color_eyre::eyre::Result;
use glob::glob;
use regex::Regex;
use std::{collections::HashSet, ffi::OsStr, path::Path, rc::Rc};

use crate::utils::RetrieveSampleIds;

/// .
///
/// # Panics
///
/// Panics if .
///
/// # Errors
///
/// This function will return an error if .
pub fn find_files(search_dir: &Path, fastq_suffix: &str) -> Result<Vec<Rc<Path>>> {
    // define the full pattern
    let pattern = format!("{}/*{}", &search_dir.display(), &fastq_suffix);

    // iterate through entries and make sure they aren't symlinks
    let fastq_paths: Vec<Rc<Path>> = glob(&pattern)?
        .filter(|entry| entry.is_ok())
        .map(|x| Rc::from(x.unwrap()))
        .collect();

    Ok(fastq_paths)
}

impl RetrieveSampleIds for SeqPlatform {
    fn retrieve_samples(&self, file_paths: &[Rc<Path>]) -> Result<HashSet<Rc<str>>> {
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
        sample_id: &'a Rc<str>,
        fastq_paths: &'a [Rc<Path>],
    ) -> Result<String>;
}

impl CollectByPlatform for SeqPlatform {
    fn collect_by_platform<'a>(
        &self,
        sample_id: &'a Rc<str>,
        fastq_paths: &'a [Rc<Path>],
    ) -> Result<String> {
        match self {
            SeqPlatform::Illumina => {
                // figure out which FASTQ files go with the provided sample_id
                let sample_fastqs: Vec<&str> = fastq_paths
                    .iter()
                    .map(|x| x.to_str().unwrap_or(""))
                    .filter(|x| x.contains(sample_id.as_ref()))
                    .collect();

                // pull out the R1 and R2 FASTQ files
                let fastq1 = sample_fastqs
                    .iter()
                    .find(|x| x.contains("R1"))
                    .ok_or("No fastq file with 'R1' found")
                    .unwrap();
                let fastq2 = sample_fastqs
                    .iter()
                    .find(|x| x.contains("R2"))
                    .ok_or("No fastq file with 'R2' found")
                    .unwrap();

                // instantiate an illumina line and return it
                Ok([sample_id.as_ref(), fastq1, fastq2].join(","))
            }
            SeqPlatform::Nanopore => {
                // pull out the barcode
                let barcode = if sample_id.starts_with('0') {
                    sample_id.replace("barcode", "").chars().skip(1).collect()
                } else {
                    sample_id.replace("barcode", "")
                };

                // instantiate a Nanopore line and return it
                Ok([sample_id.as_ref(), &barcode].join(","))
            }
        }
    }
}

/// .
pub fn concat_lines(
    sample_ids: &HashSet<Rc<str>>,
    fastq_paths: &[Rc<Path>],
    platform: &SeqPlatform,
) -> Vec<String> {
    sample_ids
        .iter()
        .filter_map(|x| platform.collect_by_platform(x, fastq_paths).ok())
        .collect::<Vec<String>>()
}

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn give_a_sheet(
    input_dir: &Path,
    fastq_ext: &str,
    platform: &SeqPlatform,
    output_prefix: &Option<String>,
) -> Result<()> {
    let fastq_paths = find_files(input_dir, fastq_ext)?;
    let sample_ids = &platform.retrieve_samples(&fastq_paths)?;
    let lines = concat_lines(sample_ids, &fastq_paths, platform);
    let header = "sample,fastq_1,fastq_2";
    write_lines(&lines, header, output_prefix)
}
