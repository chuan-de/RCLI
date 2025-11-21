pub use self::{base64::Base64Format, csv::OutputFormat,text::TextSignFormat};
pub use self::{base64::Base64SubCommand, csv::CsvOpts, genpass::GenPassOpts, text::TextSubCommand};
use clap::Parser;
use std::path::Path;
mod base64;
mod csv;
mod genpass;
mod text;

#[derive(Debug, Parser)]
#[command(name="rcli",version,author,about,long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Show CSV Or Convert CSV To Other Formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a password for RCL")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(TextSubCommand),
}

pub fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File does not exist"));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
    }
}
