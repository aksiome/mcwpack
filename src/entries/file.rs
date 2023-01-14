use std::fs;
use std::path::{PathBuf, Path};

use anyhow::Result;

use crate::config::Config;
use crate::entries::Entry;

pub struct FileEntry {
    path: PathBuf,
}

impl FileEntry {
    pub fn new(path: &Path) -> Option<Self> {
        path.is_file().then(
            || Self { path: path.to_owned() }
        )
    }
}

impl Entry for FileEntry {
    fn package(&self, _: &Config, to: &Path) -> Result<()> {
        let to = to.to_owned().join(&self.path);
        fs::create_dir_all(&to.parent().unwrap())?;
        fs::copy(&self.path, &to)?;
        Ok(())
    }
}
