use std::fs;
use std::path::{PathBuf, Path};

use anyhow::{Result, Context};

use crate::config::Config;
use crate::entries::Entry;
use crate::models::region::Region;
use crate::utils;

pub struct RegionEntry {
    path: PathBuf,
}

impl RegionEntry {
    pub fn new(path: &Path) -> Option<Self> {
        (path.is_file() && path.extension().map_or(false, |ext| ext == "mca")).then(
            || Self { path: path.to_owned() }
        )
    }
}

impl Entry for RegionEntry {
    fn package(&self, config: &Config, to: &Path) -> Result<()> {
        log::info!("processing region ({})", self.path.to_string_lossy());
        let to = to.to_owned().join(&self.path);
        fs::create_dir_all(to.parent().unwrap())?;
        if self.path.metadata()?.len() <= 8192 {
            log::info!("skipped empty region ({})", self.path.to_string_lossy());
            return Ok(());
        }
        match config.clean_chunks {
            true => Region::load(&self.path)
                .with_context(|| format!("could not read region ({})", self.path.to_string_lossy()))?
                .write_cleaned(&to)
                .with_context(|| format!("could not process region ({})", self.path.to_string_lossy())),
            false => utils::copy_file(&self.path, &to)
        }
    }
}
