use super::verify_input_file;
use anyhow::anyhow;
use clap::Parser;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Base64 Encode")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Base64 Decode")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long,value_parser=verify_input_file,default_value = "-")]
    pub input: String,
    #[arg(long, value_parser=parse_base64_format,default_value = "standard")]
    pub format: Base64Format,
}
#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long,value_parser=verify_input_file,default_value = "-")]
    pub input: String,
    #[arg(long, value_parser=parse_base64_format,default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Copy, Clone)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(s: &str) -> Result<Base64Format, anyhow::Error> {
    s.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow!("Unknown base64 format: {}", s)),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
