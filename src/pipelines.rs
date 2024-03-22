use color_eyre::eyre::Result;
use std::{collections::HashSet, fmt, path::Path, rc::Rc};

use crate::viralrecon::{self, CollectByPlatform};
use clap::ValueEnum;

#[derive(Debug, Clone)]
pub enum SupportedPipelines {
    ViralRecon,
    // Pangenome,
    // FetchNGS,
    // Demultiplex,
    // TaxProfiler,
    // MAG,
    // Sarek,
    // NanoSeq,
}

impl fmt::Display for SupportedPipelines {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SupportedPipelines::ViralRecon => "nf-core/viralrecon",
                // SupportedPipelines::Pangenome => "nf-core/pangenome",
                // SupportedPipelines::FetchNGS => "nf-core/fetchngs",
                // SupportedPipelines::Demultiplex => "nf-core/demultiplex",
                // SupportedPipelines::TaxProfiler => "nf-core/taxprofiler",
                // SupportedPipelines::MAG => "nf-core/mag",
                // SupportedPipelines::Sarek => "nf-core/sarek",
                // SupportedPipelines::NanoSeq => "nf-core/nanoseq",
            }
        )
    }
}

#[derive(ValueEnum, Debug, Clone)]
pub enum SeqPlatform {
    Illumina,
    Nanopore,
}

impl fmt::Display for SeqPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SeqPlatform::Illumina => "illumina",
                SeqPlatform::Nanopore => "nanopore",
            }
        )
    }
}

pub trait FindFiles {
    fn find_files(&self, search_dir: &Path, fastq_suffix: &str) -> Result<Vec<Rc<Path>>>;
}

impl FindFiles for SupportedPipelines {
    fn find_files(&self, search_dir: &Path, fastq_suffix: &str) -> Result<Vec<Rc<Path>>> {
        match self {
            SupportedPipelines::ViralRecon => {
                viralrecon::find_viralrecon_files(search_dir, fastq_suffix)
            } // SupportedPipelines::Pangenome => "nf-core/pangenome",
              // SupportedPipelines::FetchNGS => "nf-core/fetchngs",
              // SupportedPipelines::Demultiplex => "nf-core/demultiplex",
              // SupportedPipelines::TaxProfiler => "nf-core/taxprofiler",
              // SupportedPipelines::MAG => "nf-core/mag",
              // SupportedPipelines::Sarek => "nf-core/sarek",
              // SupportedPipelines::NanoSeq => "nf-core/nanoseq",
        }
    }
}

// TODO:
// this needs to implemented for pipeline variant and seq platform variant
pub trait RetrieveSampleIds {
    fn retrieve_samples(&self, file_paths: &[Rc<Path>]) -> HashSet<Rc<str>>;
}

// TODO:
// this needs to implemented for pipeline variant and seq platform variant
pub trait ConcatLines {
    fn concat_lines(
        &self,
        sample_ids: &HashSet<Rc<str>>,
        fastq_paths: &[Rc<Path>],
        platform: &SeqPlatform,
    ) -> Vec<String>;
}

impl ConcatLines for SupportedPipelines {
    fn concat_lines(
        &self,
        sample_ids: &HashSet<Rc<str>>,
        fastq_paths: &[Rc<Path>],
        platform: &SeqPlatform,
    ) -> Vec<String> {
        match self {
            SupportedPipelines::ViralRecon => sample_ids
                .iter()
                .filter_map(|x| platform.collect_by_platform(x, fastq_paths).ok())
                .collect::<Vec<String>>(),
        }
    }
}

pub trait GiveASheet {
    // self is an enum specifying the pipeline, search dir is the directory to search
    // for input data, fastq_suffix defaults to just ".fastq.gz",
    fn give_a_sheet(
        &self,
        search_dir: &Path,
        fastq_suffix: &str,
        platform: &SeqPlatform,
    ) -> Result<()>;
}

impl GiveASheet for SupportedPipelines {
    fn give_a_sheet(
        &self,
        search_dir: &Path,
        fastq_suffix: &str,
        platform: &SeqPlatform,
    ) -> Result<()> {
        match self {
            SupportedPipelines::ViralRecon => {
                let fastq_paths = &self.find_files(search_dir, fastq_suffix)?;
                let sample_ids: &HashSet<Rc<str>> = &platform.retrieve_samples(&fastq_paths);
                let lines = &self.concat_lines(sample_ids, fastq_paths, &platform);

                for line in lines {
                    eprintln!("{}", line);
                }

                Ok(())
            } // SupportedPipelines::Pangenome => "nf-core/pangenome",
              // SupportedPipelines::FetchNGS => "nf-core/fetchngs",
              // SupportedPipelines::Demultiplex => "nf-core/demultiplex",
              // SupportedPipelines::TaxProfiler => "nf-core/taxprofiler",
              // SupportedPipelines::MAG => "nf-core/mag",
              // SupportedPipelines::Sarek => "nf-core/sarek",
              // SupportedPipelines::NanoSeq => "nf-core/nanoseq",
        }
    }
}
