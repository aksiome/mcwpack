use std::fs;
use std::path::{PathBuf, Path};

use anyhow::Result;

use crate::config::Config;
use crate::entries::Entry;
use crate::utils;

pub struct DataPackEntry {
    path: PathBuf,
}

impl DataPackEntry {
    pub fn new(path: &Path) -> Option<Self> {
        path.parent().map_or(false, |dir| dir.ends_with("datapacks")).then(
            || Self { path: path.to_owned() }
        )
    }
}

impl Entry for DataPackEntry {
    fn package(&self, config: &Config, to: &Path) -> Result<()> {
        log::info!("processing datapack ({})", self.path.to_string_lossy());
        let to = to.to_owned().join(&self.path);
        fs::create_dir_all(to.parent().unwrap())?;
        match (self.path.is_file(), config.zip_datapacks) {
            (true, _) => utils::copy_file(&self.path, &to),
            (false, false) => utils::copy_dir(&self.path, &to),
            (false, true) => utils::create_zip(&self.path, &to),
        }
    }
}
