use crate::cli::Cli;
use clap::Parser;
use cli::Commands;
use color_eyre::eyre::Result;

pub mod cli;
pub mod scrnaseq;
pub mod utils;
pub mod viralrecon;

/// .
///
/// # Errors
///
/// This function will return an error if .
fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Viralrecon {
            input_dir,
            fastq_ext,
            platform,
            output_prefix,
        }) => {
            viralrecon::give_a_sheet(input_dir, fastq_ext, platform, output_prefix)?;
        }
        Some(Commands::Scrnaseq {
            input_dir,
            fastq_ext,
            expected_cells,
            output_prefix,
        }) => {
            scrnaseq::give_a_sheet(input_dir, fastq_ext, expected_cells, output_prefix)?;
        }
        None => {
            eprintln!("{}\n", cli::INFO);
            std::process::exit(0);
        }
    }

    Ok(())
}
