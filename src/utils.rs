use color_eyre::eyre::Result;
use std::{collections::HashSet, fmt, fs::File, io::BufWriter, io::Write, path::Path, rc::Rc};

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

pub trait RetrieveSampleIds {
    fn retrieve_samples(&self, file_paths: &[Rc<Path>]) -> Result<HashSet<Rc<str>>>;
}

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn write_lines(lines: &[String], header: &str, output_prefix: &Option<String>) -> Result<()> {
    let out_name = match output_prefix {
        Some(prefix) => format!("{}_samplesheet.csv", prefix),
        None => String::from("samplesheet.csv"),
    };

    let file = File::create(out_name)?;
    let mut buf_writer = BufWriter::new(file);

    writeln!(buf_writer, "{}", header)?;
    lines
        .iter()
        .try_for_each(|line| writeln!(buf_writer, "{}", line))?;

    Ok(())
}
