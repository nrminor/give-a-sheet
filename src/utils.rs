use std::{collections::HashSet, fmt, path::Path, rc::Rc};

use clap::ValueEnum;

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

// TODO:
// this needs to implemented for pipeline variant and seq platform variant
pub trait RetrieveSampleIds {
    fn retrieve_samples(&self, file_paths: &[Rc<Path>]) -> HashSet<Rc<str>>;
}
