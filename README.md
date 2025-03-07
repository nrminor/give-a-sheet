# Give a sheet! A command line tool for generating samplesheets for [nf-core](https://nf-co.re/) pipelines

[![Open Source Starter Files](https://github.com/nrminor/give-a-sheet/actions/workflows/open-source-starter.yml/badge.svg)](https://github.com/nrminor/give-a-sheet/actions/workflows/open-source-starter.yml) [![Rust CI](https://github.com/nrminor/give-a-sheet/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/nrminor/give-a-sheet/actions/workflows/rust-ci.yml) ![Crates.io](https://img.shields.io/crates/v/give_a_sheet) ![Crates.io](https://img.shields.io/crates/d/give_a_sheet)

`give_a_sheet` generates type-validated, error-free input samplesheets for the nf-core pipeline you want to run. Pipeline support is limited to [`viralrecon`](https://nf-co.re/viralrecon) and [`scrnaseq`](https://nf-co.re/scrnaseq) at this stage, but support for additional pipelines will be added gradually in the future.

```
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


Usage: giveasheet [OPTIONS] [COMMAND]

Commands:
  scrnaseq    Generate an input samplesheet for `nf-core/scrnaseq`. [aliases: sc, scr]
  viralrecon  Generate an input samplesheet for `nf-core/viralrecon`. [aliases: vr, virrec, vrc]
  help        Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
  -V, --version     Print version
```

### Installation

`give_a_sheet` is currently distributed through [crates.io](https://crates.io/), the Rust package repository. The easiest way to install it on your machine is to [install the Rust toolchain](https://www.rust-lang.org/tools/install) and then `cargo install` it in the command line, like so:

```zsh
cargo install give_a_sheet
```

This will compile the tool locally and make it available. Try it out with `give_a_sheet -h`,
