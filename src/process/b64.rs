use std::string::String;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use std::fs::File;
use std::io::Read;
use anyhow::Error;
use base64::engine::general_purpose::STANDARD;
use crate::cli::Base64Format;

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader: get_reader(input) ;
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(buffer),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buffer),
    };
    println!("{}", encoded);
    Ok(())
}

fn get_reader(input: &str) -> Result<Box<dyn Read>,Error> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    let buffer = buffer.trim();
    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buffer)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buffer)?,
    };
    let decoded = String::from_utf8(decoded)?;
    println!("{:?}", decoded);
    Ok(())
}
