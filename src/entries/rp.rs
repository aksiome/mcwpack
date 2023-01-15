use std::fs;
use std::path::{PathBuf, Path};

use anyhow::Result;

use crate::config::Config;
use crate::entries::Entry;
use crate::utils;

pub struct ResourcePackEntry {
    path: PathBuf,
}

impl ResourcePackEntry {
    pub fn new(path: &Path) -> Self {
        Self { path: path.to_owned() }
    }
}

impl Entry for ResourcePackEntry {
    fn package(&self, _: &Config, to: &Path) -> Result<()> {
        let to = to.to_owned().join("./resources.zip");
        fs::create_dir_all(&to.parent().unwrap())?;
        Ok(match self.path.is_file() {
            true => fs::copy(&self.path, &to).map(|_| ())?,
            false => utils::create_zip(&self.path, &to)?,
        })
    }
}
