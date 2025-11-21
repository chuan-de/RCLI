use super::verify_input_file;
use clap::Parser;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign a message with a private key")]
    Sign(TextSignOpts),
    #[command(about = "verify a signed message")]
    Verify(TextVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long,value_parser=verify_input_file,default_value = "-")]
    pub input: String,

    #[arg(short, long,value_parser=verify_input_file)]
    pub key: String,

    #[arg(long,default_value = "blake3",value_parser=parse_text_sign_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long,value_parser=verify_input_file,default_value = "-")]
    pub input: String,
    #[arg(short, long,value_parser=verify_input_file)]
    pub key: String,
    #[arg(short, long)]
    pub sig: String,
    #[arg(long,default_value = "blake3",value_parser=parse_text_sign_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Copy, Clone)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_text_sign_format(s: &str) -> Result<TextSignFormat, String> {
    s.parse()
}

impl FromStr for TextSignFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
         match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(format!("Invalid format {}", s)),
         }
    }
}
impl std::fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextSignFormat::Blake3 => write!(f, "blake3"),
            TextSignFormat::Ed25519 => write!(f, "ed25519"),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}
