use std::fmt;

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

impl Default for SupportedPipelines {
    fn default() -> Self {
        Self::ViralRecon
    }
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
