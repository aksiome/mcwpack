use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use anyhow::Result;
use fastanvil::ChunkData;
use fastanvil::Region as Chunks;
use fastnbt::Value;
use serde::{Serialize, Deserialize};

use crate::config::Config;

type Compound = HashMap<String, fastnbt::Value>;

pub struct Region<'a> {
    config: &'a Config,
    chunks: Chunks<File>,
}

#[derive(Serialize, Deserialize)]
struct Chunk {
    #[serde(rename = "Sections")]
    poi: Option<Compound>,
    #[serde(rename = "Entities")]
    entities: Option<Vec<Compound>>,
    sections: Option<Vec<Section>>,
    block_entities: Option<Vec<Compound>>,
    #[serde(flatten)]
    other: Compound,
}

#[derive(Serialize, Deserialize)]
struct Section {
    block_states: Option<BlockStates>,
    #[serde(flatten)]
    other: Compound,
}

#[derive(Serialize, Deserialize)]
struct BlockStates {
    palette: Vec<PaletteItem>,
    #[serde(flatten)]
    other: Compound,
}

#[derive(Serialize, Deserialize)]
struct PaletteItem {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Properties")]
    properties: Option<Value>,
}

impl Chunk {
    pub fn new(data: &ChunkData) -> Result<Chunk> {
        Ok(fastnbt::from_bytes(&data.data)?)
    }

    pub fn is_chunk_empty(&self, ignored_blocks: &Vec<String>) -> bool {
        self.other.get("Status").map_or(false, |v| v != "full") ||
        !self.block_entities.as_ref().map_or(false, |chunk| !chunk.is_empty()) &&
        !self.entities.as_ref().map_or(false, |chunk| !chunk.is_empty()) &&
        !self.poi.as_ref().map_or(false, |chunk| !chunk.is_empty()) &&
        !self.sections.as_ref().map_or(false, |chunk| {
            for section in chunk.iter() {
                if let Some(block_states) = &section.block_states {
                    for item in &block_states.palette {
                        if !ignored_blocks.contains(&item.name) {
                            return true;
                        }
                    }
                }
            }
            false
        })
    }
}

impl<'a> Region<'a> {
    pub fn load(from: &Path, config: &'a Config) -> Result<Self> {
        let chunks = Chunks::from_stream(File::open(from)?)?;
        Ok(Self { chunks, config })
    }

    pub fn write_cleaned(&mut self, to: &Path) -> Result<()> {
        let mut chunks = None;
        for data in self.chunks.borrow_mut().iter() {
            let data = &data?;
            let chunk = &Chunk::new(data)?;
            if !chunk.is_chunk_empty(&self.config.ignored_blocks) {
                let ser = fastnbt::to_bytes(chunk)?;
                if chunks.is_none() {
                    let file = File::options().read(true).write(true).create(true).truncate(true).open(to)?;
                    chunks = Some(Chunks::new(file)?);
                }
                chunks.as_mut().unwrap().write_chunk(data.x, data.z, &ser)?;
            }
        }
        Ok(())
    }
}
