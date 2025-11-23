use crate::cli::Base64Format;
use crate::get_reader;
use base64::engine::general_purpose::STANDARD;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use std::io::Read;
use std::string::String;

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut reader = get_reader(input)?;
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(buffer),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buffer),
    };
    Ok(encoded)
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut reader = get_reader(input)?;
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    let buffer = buffer.trim();
    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buffer)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buffer)?,
    };
    let decoded = String::from_utf8(decoded)?;
    Ok(decoded)
}
