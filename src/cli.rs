use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::utils::SeqPlatform;

pub const INFO: &str = r"

 ██████╗ ██╗██╗   ██╗███████╗     █████╗     ███████╗██╗  ██╗███████╗███████╗████████╗██╗
██╔════╝ ██║██║   ██║██╔════╝    ██╔══██╗    ██╔════╝██║  ██║██╔════╝██╔════╝╚══██╔══╝██║
██║  ███╗██║██║   ██║█████╗      ███████║    ███████╗███████║█████╗  █████╗     ██║   ██║
██║   ██║██║╚██╗ ██╔╝██╔══╝      ██╔══██║    ╚════██║██╔══██║██╔══╝  ██╔══╝     ██║   ╚═╝
╚██████╔╝██║ ╚████╔╝ ███████╗    ██║  ██║    ███████║██║  ██║███████╗███████╗   ██║   ██╗
 ╚═════╝ ╚═╝  ╚═══╝  ╚══════╝    ╚═╝  ╚═╝    ╚══════╝╚═╝  ╚═╝╚══════╝╚══════╝   ╚═╝   ╚═╝

Give-A-Sheet: A Command Line Tool that Constructs Input Samplesheets for NF-Core Pipelines
=========================================================================================

Pipelines from nf-core simplify the specification of inputs by allowing the user to give
arbitrarily complex metadata about their samples in an input samplesheet. This means the
pipeline requirements for these samplesheets are also complex, and because Nextflow is a
dynamically typed interpreted language, you won't see that you've made a mistake in your
samplesheet until runtime (unless you download and use the excellent nf-validation!).

Give-A-Sheet handles all this for you by generating type-validated, error-free input
samplesheets for the pipeline you want to run. Pipeline support is limited at this stage,
but more pipelines will be added in the future.
";

#[derive(Parser)]
#[clap(name = "give_a_sheet")]
#[clap(about = INFO)]
#[clap(version = "v0.1.2")]
pub struct Cli {
    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(
            about = "Generate an input samplesheet for `nf-core/scrnaseq`.",
            aliases = &["sc", "scr"]
        )]
    Scrnaseq {
        /// Input directory to traverse for FASTQ files.
        #[arg(short, long, required = false)]
        input_dir: PathBuf,

        /// File extension for FASTQ files
        #[arg(short, long, required = false, default_value = ".fastq.gz")]
        fastq_ext: String,

        /// the number of cells expected
        #[arg(short, long, required = true, default_value_t = 10000)]
        expected_cells: i64,

        /// Output file prefix (the part before the `_samplesheet.csv`)
        #[arg(short, long, required = false, default_value = None)]
        output_prefix: Option<String>,
    },
    #[clap(
        about = "Generate an input samplesheet for `nf-core/viralrecon`.",
        aliases = &["vr", "virrec", "vrc"]
    )]
    Viralrecon {
        /// Input directory to traverse for FASTQ files.
        #[arg(short, long, required = false)]
        input_dir: PathBuf,

        /// File extension for FASTQ files
        #[arg(short, long, required = false, default_value = ".fastq.gz")]
        fastq_ext: String,

        /// The sequencing platform where FASTQs came from
        #[arg(short, long, required = true)]
        platform: SeqPlatform,

        /// Output file prefix (the part before the `_samplesheet.csv`)
        #[arg(short, long, required = false, default_value = None)]
        output_prefix: Option<String>,
        // /// Check a pre-existing samplesheet
        // #[arg(short, long, required = false, default_value = "samplesheet.csv")]
        // check: Option<String>,
    },
}
