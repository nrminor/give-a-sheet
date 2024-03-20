use std::{collections::HashSet, fmt, path::PathBuf, rc::Rc};

use crate::RetrieveSampleIds;

#[derive(Debug, Clone)]
pub enum ViralReconPlatforms {
    Illumina,
    Nanopore,
}

impl fmt::Display for ViralReconPlatforms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ViralReconPlatforms::Illumina => "illumina",
                ViralReconPlatforms::Nanopore => "nanopore",
            }
        )
    }
}

impl Default for ViralReconPlatforms {
    fn default() -> Self {
        Self::Illumina
    }
}

impl RetrieveSampleIds for ViralReconPlatforms {
    fn retrieve_samples(&self, file_paths: &[PathBuf]) -> HashSet<Rc<str>> {
        let illumina_patterns = ["_L001_R2_001.fastq.gz", "_L001_R1_001.fastq.gz"].join("|");
        let nanopore_patterns = [".fastq.gz"].join("|");
        match self {
            ViralReconPlatforms::Illumina => file_paths
                .into_iter()
                .map(|path| {
                    Rc::from(
                        path.file_name()
                            .unwrap()
                            .to_string_lossy()
                            .replace(&illumina_patterns, "")
                            .as_ref(),
                    )
                })
                .collect(),
            ViralReconPlatforms::Nanopore => file_paths
                .into_iter()
                .map(|path| {
                    Rc::from(
                        path.file_name()
                            .unwrap()
                            .to_string_lossy()
                            .replace(&nanopore_patterns, "")
                            .as_ref(),
                    )
                })
                .collect(),
        }
    }
}
