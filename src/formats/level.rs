use serde::{Deserialize, Serialize};

use super::nbt::{NbtFormat, Compound};

impl NbtFormat for Level {}

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
    pub fn walk_datapacks(&mut self, callback: fn(&mut String)) {
        self.walk_enabled_datapacks(callback);
        self.walk_disabled_datapacks(callback);
    }

    pub fn walk_disabled_datapacks(&mut self, callback: fn(&mut String)) {
        for datapack in self.data.datapacks.disabled.iter_mut() {
            callback(datapack);
        }
    }

    pub fn walk_enabled_datapacks(&mut self, callback: fn(&mut String)) {
        for datapack in self.data.datapacks.enabled.iter_mut() {
            callback(datapack);
        }
    }
}
