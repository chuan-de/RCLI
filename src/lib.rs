mod opts;
mod process;

pub use opts::{Opts, Subcommand,GenPassOpts};

pub use process::process_csv;
pub use process::process_genpass;

