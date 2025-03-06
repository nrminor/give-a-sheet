#![allow(clippy::pedantic, clippy::perf)]

use std::{collections::HashSet, fmt, path::PathBuf, rc::Rc};

use clap::ValueEnum;
use color_eyre::Result;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GiveASheetError<P: NfCore + Sized> {
    #[error("The requested pipeline `{0}` is not present in nf-core.")]
    PipelineDoesNotExist(String),
    #[error("invalid file extension requested for the pipeline {pipeline:?} (expected {expected:?}, found {found:?})")]
    InvalidFileExt {
        pipeline: P,
        expected: String,
        found: String,
    },
}

pub trait NfCore: Sized + Debug {}

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

pub trait FindInputs: NfCore {
    fn find_files(&self) -> Result<Vec<PathBuf>>;
}

pub trait RetrieveSampleIds {
    fn retrieve_samples(&self, file_paths: &[PathBuf]) -> Result<HashSet<Rc<str>>>;
}

pub trait ConcatFields: NfCore + FindInputs {
    fn concat_lines(&self, sample_ids: &HashSet<Rc<str>>, fastq_paths: &[PathBuf]) -> Vec<String>;
}

pub trait WriteCsvLines: NfCore + FindInputs + ConcatFields {
    fn write_lines(&self, lines: &[String], header: &str) -> Result<()>;
}

pub trait GiveASheet: NfCore + FindInputs + ConcatFields + WriteCsvLines {
    fn give_a_sheet(self) -> Result<()>;
}
