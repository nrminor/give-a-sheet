use crate::cli::Cli;
use clap::Parser;
use cli::Commands;
use color_eyre::eyre::Result;
use pipelines::{GiveASheet, SupportedPipelines};

pub mod cli;
pub mod pipelines;
pub mod viralrecon;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::ViralRecon {
            input_dir,
            fastq_ext,
            platform,
        }) => {
            SupportedPipelines::ViralRecon.give_a_sheet(&input_dir, &fastq_ext, platform)?;
        }
        None => {
            eprintln!("{}\n", cli::INFO);
            std::process::exit(0);
        }
    }

    Ok(())
}
