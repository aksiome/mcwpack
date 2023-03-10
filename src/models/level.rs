use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use anyhow::Result;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use serde::{Serialize, Deserialize};

type Edit = fn(&mut String);
type Compound = HashMap<String, fastnbt::Value>;

#[derive(Serialize, Deserialize)]
pub struct Level {
    #[serde(rename = "Data")]
    pub data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "LevelName")]
    pub name: String,
    #[serde(rename = "Player")]
    pub player: Compound,
    #[serde(rename = "DataPacks")]
    pub datapacks: DataPacks,
    #[serde(flatten)]
    other: Compound,
}

#[derive(Serialize, Deserialize)]
pub struct DataPacks {
    #[serde(rename = "Enabled")]
    pub enabled: Vec<String>,
    #[serde(rename = "Disabled")]
    pub disabled: Vec<String>,
}

impl Level {
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

    pub fn update_all_datapacks(&mut self, callback: Edit) {
        self.update_enabled_datapacks(callback);
        self.update_disabled_datapacks(callback);
    }

    pub fn update_disabled_datapacks(&mut self, callback: Edit) {
        for datapack in self.data.datapacks.disabled.iter_mut() {
            callback(datapack);
        }
    }

    pub fn update_enabled_datapacks(&mut self, callback: Edit) {
        for datapack in self.data.datapacks.enabled.iter_mut() {
            callback(datapack);
        }
    }
}
