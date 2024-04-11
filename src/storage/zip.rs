use std::fs::File;
use std::io::{Cursor, Seek, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use anyhow::Result;
use zip::result::ZipResult;
use zip::write::FileOptions;
use zip::ZipWriter;

use super::{FilesystemStorage, InMemoryStorage, Storage};

pub struct ZipStorage<W: Write + Seek> {
    path: Option<PathBuf>,
    writer: Mutex<ZipWriter<W>>,
}

impl InMemoryStorage for ZipStorage<Cursor<Vec<u8>>> {
    fn new(buffer: &[u8]) -> Self {
        Self {
            path: None,
            writer: Mutex::new(ZipWriter::new(Cursor::new(buffer.to_vec()))),
        }
    }
}

impl FilesystemStorage for ZipStorage<File> {
    fn new(path: &Path) -> Self {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }

        Self {
            path: Some(path.to_owned()),
            writer: Mutex::new(ZipWriter::new(File::create(path).unwrap())),
        }
    }
}

impl<W: Write + Seek> ZipStorage<W> {
    pub fn finish(&mut self) -> ZipResult<W> {
        self.writer.lock().unwrap().finish()
    }
}

impl<W: Write + Seek + Send + Sync> Storage for ZipStorage<W> {
    fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    fn copy(&self, from: &Path, to: &Path) -> Result<()> {
        self.write(to, &std::fs::read(from)?)
    }

    fn write(&self, file: &Path, contents: &[u8]) -> Result<()> {
        let mut writer = self.writer.lock().unwrap();
        let name = file.strip_prefix("./").unwrap_or(file).to_string_lossy();
        writer.start_file(name, FileOptions::default().compression_level(Some(9)))?;

        Ok(writer.write_all(contents)?)
    }
}
