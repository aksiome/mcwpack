use std::path::{Path, PathBuf};

use anyhow::Result;

use super::{FilesystemStorage, Storage};

pub struct DirStorage {
    root: PathBuf,
}

impl FilesystemStorage for DirStorage {
    fn new(path: &Path) -> Self {
        Self { root: path.to_owned() }
    }
}

impl Storage for DirStorage {
    fn path(&self) -> Option<&Path> {
        Some(&self.root)
    }

    fn copy(&self, from: &Path, to: &Path) -> Result<()> {
        let to = self.root.join(to);
        std::fs::create_dir_all(to.parent().unwrap())?;
        std::fs::copy(from, to)?;

        Ok(())
    }

    fn write(&self, file: &Path, contents: &[u8]) -> Result<()> {
        let file = self.root.join(file);
        std::fs::create_dir_all(file.parent().unwrap())?;

        Ok(std::fs::write(file, contents)?)
    }
}
