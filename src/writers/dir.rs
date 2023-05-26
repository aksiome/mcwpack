use std::path::{Path, PathBuf};

use anyhow::Result;
use ignore::WalkBuilder;

use super::Writer;

pub struct DirWriter {
    root: PathBuf,
}

impl DirWriter {
    pub fn new(dir: &Path) -> Self {
        Self { root: dir.to_path_buf() }
    }
}

impl Writer for DirWriter {
    fn copy(&mut self, entry: &Path) -> Result<()> {
        if entry.is_file() {
            let path = self.root.join(entry);
            std::fs::create_dir_all(path.parent().unwrap())?;
            std::fs::copy(entry, path)?;
        } else if entry.is_dir() {
            let walker = WalkBuilder::new(entry).same_file_system(true).build();
            for entry in walker.into_iter().filter_map(|file| file.ok()) {
                let path = self.root.join(entry.path());
                std::fs::create_dir_all(path.parent().unwrap())?;
                std::fs::copy(entry.path(), path)?;
            }
        };

        Ok(())
    }

    fn write(&mut self, name: &Path, contents: Vec<u8>) -> Result<()> {
        let path = self.root.join(name);
        std::fs::create_dir_all(path.parent().unwrap())?;

        Ok(std::fs::write(path, contents)?)
    }
}
