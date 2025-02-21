use clap::Parser;
use color_eyre::eyre::Result;
use give_a_sheet::cli::{self, Cli, Commands};
use libsamplesheet::{prelude::GiveASheet, scrnaseq::ScrnaSeq, viralrecon::ViralRecon};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Viralrecon {
            input_dir,
            fastq_ext,
            platform,
            output_prefix,
        }) => {
            ViralRecon::new(input_dir, fastq_ext, platform, output_prefix).give_a_sheet()?;
        }
        Some(Commands::Scrnaseq {
            input_dir,
            fastq_ext,
            expected_cells,
            output_prefix,
        }) => {
            ScrnaSeq::new(input_dir, fastq_ext, expected_cells, output_prefix).give_a_sheet()?;
        }
        None => {
            eprintln!("{}\n", cli::INFO);
            std::process::exit(0);
        }
    }

    Ok(())
}
