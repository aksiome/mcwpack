use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use anyhow::Result;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

pub type Compound = HashMap<String, fastnbt::Value>;

pub trait NbtFormat {
    fn load(from: &Path) -> Result<Self>
    where
        Self: Sized + serde::de::DeserializeOwned,
    {
        let file = File::open(from)?;
        let mut decoder = GzDecoder::new(file);
        let mut bytes = vec![];
        decoder.read_to_end(&mut bytes)?;

        Ok(fastnbt::from_bytes(&bytes)?)
    }

    fn to_bytes(&self) -> Result<Vec<u8>>
    where
        Self: serde::Serialize,
    {
        let bytes = fastnbt::to_bytes(&self)?;
        let mut encoder = GzEncoder::new(vec![], Compression::fast());
        encoder.write_all(&bytes)?;

        Ok(encoder.finish()?)
    }
}
