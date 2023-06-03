use serde::{Deserialize, Serialize};

use super::nbt::{NbtFormat, Compound};

impl NbtFormat for Scoreboard {}

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
