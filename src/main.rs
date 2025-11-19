use RCLI::{process_csv, Opts, Subcommand};
use clap::Parser;
use csv::Reader;
use std::fs;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
    }
    Ok(())
}
