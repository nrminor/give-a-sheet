use anyhow::Result;
use std::{collections::HashSet, fmt, path::Path, rc::Rc};

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

pub trait RetrieveSampleIds {
    fn retrieve_samples(&self, file_paths: &[Rc<Path>]) -> HashSet<Rc<str>>;
}

#[derive(Debug)]
pub struct PairedFiles<'a> {
    pub basename: &'a str,
    pub r1_file: &'a str,
    pub r2_file: &'a str,
}

#[derive(Debug)]
pub struct NanoporeFiles<'a> {
    pub basename: &'a str,
    pub barcode: &'a usize,
}

pub fn collect_fields<'a>(
    sample_id: &'a Rc<str>,
    fastq_paths: &'a [Rc<Path>],
) -> Result<PairedFiles<'a>> {
    // figure out which FASTQ files go with the provided sample_id
    let sample_fastqs: Vec<&str> = fastq_paths
        .into_iter()
        .map(|x| match x.to_str() {
            Some(path_str_slice) => path_str_slice,
            None => &"",
        })
        .filter(|x| x.contains(sample_id.as_ref()))
        .collect();

    // pull out the R1 and R2 FASTQ files
    let fastq1: &str = &sample_fastqs
        .iter()
        .filter(|x| x.contains("R1"))
        .collect::<Vec<_>>()
        .first()
        .unwrap();
    let fastq2: &str = &sample_fastqs
        .iter()
        .filter(|x| x.contains("R1"))
        .collect::<Vec<_>>()
        .first()
        .unwrap();

    // instantiate an illumina line and return it
    Ok(PairedFiles {
        basename: sample_id,
        r1_file: fastq1,
        r2_file: fastq2,
    })
}
