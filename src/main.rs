use anyhow::Result;
use glob::glob;
use std::{collections::HashSet, path::PathBuf, rc::Rc};

pub mod cli;

enum _ViralReconPlatforms {
    Illumina,
    Nanopore,
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

fn main() -> Result<()> {
    // these settings will be replaced with clap command line args
    let search_dir = String::from("fastq/");
    let fastq_suffix = String::from(".fastq.gz");
    let pipeline = "viralrecon";
    let mode = "illumina";

    // TODO: temporary dbg macros to callout information that will be
    // replaced with clap args
    dbg!("Directory of FASTQs specified: {}", &search_dir);
    dbg!("Pipeline selected: {}", pipeline);
    dbg!("Mode selected: {}", mode);

    // define the full pattern
    let pattern = format!("{}/*{}", &search_dir, &fastq_suffix);

    // iterate through entries and make sure they aren't symlinks
    let fastq_paths: Vec<PathBuf> = glob(&pattern)?
        .filter_map(|entry| entry.ok().map(PathBuf::from))
        .collect();

    // separate out the basenames of the FASTQs
    let _sample_ids: HashSet<Rc<str>> = fastq_paths
        .into_iter()
        .map(|path| Rc::from(path.file_name().unwrap().to_string_lossy().as_ref()))
        .collect();

    Ok(())
}
