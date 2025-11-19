use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    #[serde(rename = "Title")]
    title: String,
    #[serde(rename = "Identifier")]
    identifier: String,
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Subject")]
    subject: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Notes")]
    notes: String,
    #[serde(rename = "Creator")]
    creator: String,
    #[serde(rename = "Accession")]
    accession: String,
    #[serde(rename = "Accession No")]
    accession_no: f64,
    #[serde(rename = "Reproduction")]
    reproduction: String,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut rdr = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);

    for result in rdr.deserialize() {
        let record: Player = result?;
        ret.push(record);
    }

    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;
    Ok(())
}
