use crate::pipelines::SeqPlatform;
use color_eyre::eyre::Result;
use glob::glob;
use std::{collections::HashSet, ffi::OsStr, path::Path, rc::Rc};

use crate::pipelines::RetrieveSampleIds;

pub fn find_viralrecon_files(search_dir: &Path, fastq_suffix: &str) -> Result<Vec<Rc<Path>>> {
    // define the full pattern
    let pattern = format!("{}/*{}", &search_dir.display(), &fastq_suffix);

    // iterate through entries and make sure they aren't symlinks
    let fastq_paths: Vec<Rc<Path>> = glob(&pattern)?
        .filter(|entry| entry.is_ok())
        .map(|x| x.unwrap())
        .map(|x| Rc::from(x))
        .collect();

    Ok(fastq_paths)
}

impl RetrieveSampleIds for SeqPlatform {
    fn retrieve_samples(&self, file_paths: &[Rc<Path>]) -> HashSet<Rc<str>> {
        match self {
            // handle paired end FASTQ files for Illumina
            SeqPlatform::Illumina => file_paths
                .into_iter()
                .map(|path| {
                    Rc::from(
                        path.file_name()
                            .unwrap_or(OsStr::new(""))
                            .to_string_lossy()
                            .replace("_L001_R1_001.fastq.gz", "")
                            .replace("_L001_R2_001.fastq.gz", "")
                            .as_ref(),
                    )
                })
                .collect(),
            // handle per-barcode single FASTQs for Nanopore
            SeqPlatform::Nanopore => file_paths
                .into_iter()
                .map(|path| {
                    Rc::from(
                        path.file_name()
                            .unwrap_or(OsStr::new(""))
                            .to_string_lossy()
                            .replace(".fastq.gz", "")
                            .as_ref(),
                    )
                })
                .collect(),
        }
    }
}

pub trait CollectByPlatform {
    fn collect_by_platform<'a>(
        &self,
        sample_id: &'a Rc<str>,
        fastq_paths: &'a [Rc<Path>],
    ) -> Result<String>;
}

impl CollectByPlatform for SeqPlatform {
    fn collect_by_platform<'a>(
        &self,
        sample_id: &'a Rc<str>,
        fastq_paths: &'a [Rc<Path>],
    ) -> Result<String> {
        match self {
            SeqPlatform::Illumina => {
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
                    .find(|x| x.contains("R1"))
                    .ok_or("No fastq file with 'R1' found")
                    .unwrap();
                let fastq2: &str = &sample_fastqs
                    .iter()
                    .find(|x| x.contains("R2"))
                    .ok_or("No fastq file with 'R2' found")
                    .unwrap();

                // instantiate an illumina line and return it
                Ok(vec![sample_id, fastq1, fastq2].join(","))
            }
            SeqPlatform::Nanopore => {
                // figure out which FASTQ files go with the provided sample_id
                let barcode_fastq = fastq_paths
                    .iter()
                    .filter_map(|x| x.to_str())
                    .find(|x| x.contains(sample_id.as_ref()))
                    .ok_or("Sample ID no longer matches a FASTQ.")
                    .unwrap_or("");

                // instantiate an illumina line and return it
                Ok(vec![sample_id.as_ref(), barcode_fastq].join(","))
            }
        }
    }
}
