use crate::cli::Cli;
use clap::Parser;
use cli::Commands;
use color_eyre::eyre::Result;

pub mod cli;
pub mod scrnaseq;
pub mod utils;
pub mod viralrecon;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::ViralRecon {
            input_dir,
            fastq_ext,
            platform,
        }) => {
            viralrecon::give_a_sheet(input_dir, fastq_ext, platform)?;
        }
        Some(Commands::SCRNAseq {
            input_dir,
            fastq_ext,
            expected_cells,
        }) => {
            scrnaseq::give_a_sheet(input_dir, fastq_ext, &expected_cells)?;
        }
        None => {
            eprintln!("{}\n", cli::INFO);
            std::process::exit(0);
        }
    }

    Ok(())
}
