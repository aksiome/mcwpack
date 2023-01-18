use std::fs;
use std::path::{PathBuf, Path};

use anyhow::Result;

use crate::config::Config;
use crate::utils;

use super::Entry;
use super::WorldEntry;

pub struct FileEntry {
    path: PathBuf,
}

impl FileEntry {
    pub fn try_create(path: &Path) -> Option<WorldEntry> {
        path.is_file().then(
            || WorldEntry::File(Self { path: path.to_owned() })
        )
    }
}

impl Entry for FileEntry {
    fn package(&self, _: &Config, to: &Path) -> Result<()> {
        log::info!("processing file ({})", self.path.to_string_lossy());
        let to = to.to_owned().join(&self.path);
        fs::create_dir_all(to.parent().unwrap())?;
        utils::copy_file(&self.path, &to)
    }
}
