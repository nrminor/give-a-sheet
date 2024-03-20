use anyhow::Result;
use glob::glob;
use pipelines::{collect_fields, PairedFiles, RetrieveSampleIds};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::viralrecon::ViralReconPlatforms;

pub mod cli;
pub mod pipelines;
pub mod viralrecon;

// fn _write_illumina_line(illum_line: Vec<PairedFiles>) -> Result<()> {
//     for entry in illum_line {
//         println!(
//             "{},{},{}",
//             entry.basename,
//             entry.r1_file.display(),
//             entry.r2_file.display()
//         );
//     }

//     Ok(())
// }

fn main() -> Result<()> {
    // these settings will be replaced with clap command line args
    let search_dir =
        PathBuf::from("/Users/nickminor/Documents/dholk_experiments/29849/run_inputs/29758/fastq");
    let fastq_suffix = String::from(".fastq.gz");
    let mode = ViralReconPlatforms::Illumina;

    // define the full pattern
    let pattern = format!("{}/*{}", &search_dir.display(), &fastq_suffix);

    // iterate through entries and make sure they aren't symlinks
    let fastq_paths: Vec<Rc<Path>> = glob(&pattern)?
        .filter(|entry| entry.is_ok())
        .map(|x| x.unwrap())
        .map(|x| Rc::from(x))
        .collect();

    // separate out the basenames of the FASTQs
    let sample_ids: &HashSet<Rc<str>> = &mode.retrieve_samples(&fastq_paths);

    // collect the filenames associated with each basename
    let lines: Vec<PairedFiles> = sample_ids
        .into_iter()
        .map(|x| collect_fields(x, &fastq_paths))
        .filter_map(Result::ok)
        .collect();

    for line in lines {
        println!("{},{},{}", line.basename, line.r1_file, line.r2_file);
    }

    Ok(())
}
