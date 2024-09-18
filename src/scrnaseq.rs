use regex::Regex;
use std::{collections::HashSet, ffi::OsStr, path::Path, rc::Rc};
use tracing::warn;

use crate::utils::write_lines;
pub use crate::viralrecon::find_files;
use color_eyre::eyre::Result;

/// .
///
/// # Panics
///
/// Panics if .
fn retrieve_samples(file_paths: &[Rc<Path>]) -> Result<HashSet<Rc<str>>> {
    let illumina_pattern = Regex::new(r"_(?:S\d+_)?(?:L\d+_)?R\d+(?:_\d+)?\.fastq\.gz$")?;

    let samples = file_paths
        .iter()
        .map(|path| {
            Rc::from(
                path.file_name()
                    .unwrap_or(OsStr::new(""))
                    .to_string_lossy()
                    .as_ref(),
            )
        })
        .map(|x| {
            let id = illumina_pattern.replace_all(&x, "").to_string();
            Rc::from(id)
        })
        .collect();

    Ok(samples)
}

fn check_sample_ids(sample_ids: &HashSet<Rc<str>>) {
    for id in sample_ids {
        if id.chars().count() >= 64 {
            warn!("Sample id {} is 64 or more characters long, which is maximum enforced by some of the SCRNAseq aligners. SCRNAseq may crash if you don't manually shorten the id in your samplesheet.", id)
        }
    }
}

/// .
///
/// # Panics
///
/// Panics if .
///
/// # Errors
///
/// This function will return an error if .
fn collect_per_sample(
    sample_id: &Rc<str>,
    fastq_paths: &[Rc<Path>],
    expected_cells: &i64,
) -> Result<String> {
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

    let cell_str = expected_cells.to_string();

    // instantiate an illumina line and return it
    Ok([sample_id.as_ref(), fastq1, fastq2, &cell_str].join(","))
}

/// .
fn concat_lines(
    sample_ids: &HashSet<Rc<str>>,
    fastq_paths: &[Rc<Path>],
    expected_cells: &i64,
) -> Vec<String> {
    sample_ids
        .iter()
        .filter_map(|x| collect_per_sample(x, fastq_paths, expected_cells).ok())
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
    expected_cells: &i64,
    output_prefix: &Option<String>,
) -> Result<()> {
    // find the FASTQ files and separate out the unique sample IDs
    let fastq_paths = find_files(input_dir, fastq_ext)?;
    let sample_ids = retrieve_samples(&fastq_paths)?;

    // check the sample IDs for any that are too long
    check_sample_ids(&sample_ids);

    // concatenate and write the lines
    let lines = concat_lines(&sample_ids, &fastq_paths, expected_cells);
    let header = "sample,fastq_1,fastq_2,expected_cells";
    write_lines(&lines, header, output_prefix)
}
