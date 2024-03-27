use std::{collections::HashSet, ffi::OsStr, path::Path, rc::Rc};

pub use crate::viralrecon::find_files;
use color_eyre::eyre::Result;

fn retrieve_samples(file_paths: &[Rc<Path>]) -> HashSet<Rc<str>> {
    file_paths
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
        .collect()
}

fn collect_per_sample(
    sample_id: &Rc<str>,
    fastq_paths: &[Rc<Path>],
    expected_cells: &i64,
) -> Result<String> {
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

    let cell_str = expected_cells.to_string();

    // instantiate an illumina line and return it
    Ok(vec![sample_id, fastq1, fastq2, &cell_str].join(","))
}

pub fn concat_lines(
    sample_ids: &HashSet<Rc<str>>,
    fastq_paths: &[Rc<Path>],
    expected_cells: &i64,
) -> Vec<String> {
    sample_ids
        .iter()
        .filter_map(|x| collect_per_sample(x, fastq_paths, expected_cells).ok())
        .collect::<Vec<String>>()
}

pub fn give_a_sheet(input_dir: &Path, fastq_ext: &str, expected_cells: &i64) -> Result<()> {
    let fastq_paths = find_files(input_dir, fastq_ext)?;
    let sample_ids: HashSet<Rc<str>> = retrieve_samples(&fastq_paths);
    let lines = concat_lines(&sample_ids, &fastq_paths, expected_cells);

    for line in lines {
        println!("{}", line)
    }

    Ok(())
}
