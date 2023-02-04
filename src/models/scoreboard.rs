use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use anyhow::Result;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use serde::{Deserialize, Serialize};

type Compound = HashMap<String, fastnbt::Value>;

#[derive(Serialize, Deserialize)]
pub struct Scoreboard {
    pub data: Data,
    #[serde(rename = "DataVersion")]
    pub version: u16,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "PlayerScores")]
    pub scores: Vec<Score>,
    #[serde(rename = "Objectives")]
    pub objectives: Vec<Objective>,
    #[serde(flatten)]
    other: Compound,
}

#[derive(Serialize, Deserialize)]
pub struct Score {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(flatten)]
    other: Compound,
}

#[derive(Serialize, Deserialize)]
pub struct Objective {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(flatten)]
    other: Compound,
}

impl Scoreboard {
    pub fn load(from: &Path) -> Result<Self> {
        let file = File::open(from)?;
        let mut decoder = GzDecoder::new(file);
        let mut bytes = vec![];
        decoder.read_to_end(&mut bytes)?;
        Ok(fastnbt::from_bytes(&bytes)?)
    }

    pub fn write(&self, to: &Path) -> Result<()> {
        let file = File::create(to)?;
        let bytes = fastnbt::to_bytes(self)?;
        let mut encoder = GzEncoder::new(file, Compression::fast());
        encoder.write_all(&bytes)?;
        Ok(())
    }
}
