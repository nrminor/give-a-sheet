use anyhow::Result;
use glob::glob;
use std::{collections::HashSet, path::PathBuf, rc::Rc};

use crate::viralrecon::ViralReconPlatforms;

pub mod cli;
pub mod pipelines;
pub mod viralrecon;

trait RetrieveSampleIds {
    fn retrieve_samples(&self, file_paths: &[PathBuf]) -> HashSet<Rc<str>>;
}

struct _PairedFiles<'a> {
    basename: &'a str,
    r1_file: &'a PathBuf,
    r2_file: &'a PathBuf,
}

struct _NanoporeFiles<'a> {
    basename: &'a str,
    barcode: &'a usize,
}

fn _write_illumina_line(illum_line: Vec<_PairedFiles>) -> Result<()> {
    for entry in illum_line {
        println!(
            "{},{},{}",
            entry.basename,
            entry.r1_file.display(),
            entry.r2_file.display()
        );
    }

    Ok(())
}

fn _collect_files_per_sample(_fastq_paths: &[PathBuf]) -> Result<()> {
    Ok(())
}

fn main() -> Result<()> {
    // these settings will be replaced with clap command line args
    let search_dir =
        String::from("/Users/nickminor/Documents/dholk_experiments/29849/run_inputs/29758/fastq");
    let fastq_suffix = String::from(".fastq.gz");
    let mode = ViralReconPlatforms::Illumina;

    // define the full pattern
    let pattern = format!("{}/*{}", &search_dir, &fastq_suffix);

    // iterate through entries and make sure they aren't symlinks
    let fastq_paths: Vec<PathBuf> = glob(&pattern)?
        .filter_map(|entry| entry.ok().map(PathBuf::from))
        .collect();

    // separate out the basenames of the FASTQs
    let sample_ids: &HashSet<Rc<str>> = &mode.retrieve_samples(&fastq_paths);

    // collect the filenames associated with each basename
    // let filenames = sample_ids.into_iter()

    dbg!(&sample_ids);

    Ok(())
}
