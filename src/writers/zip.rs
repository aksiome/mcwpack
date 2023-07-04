use std::fs::File;
use std::io::{Write, Seek, Cursor};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use anyhow::Result;
use ignore::WalkBuilder;
use zip::result::ZipResult;
use zip::write::FileOptions;
use zip::ZipWriter as Zip;

use crate::config::ExtraFile;
use super::Writer;

pub struct ZipWriter<W: Write + Seek> {
    zip: Mutex<Zip<W>>,
    write_dir: Option<PathBuf>,
}

impl ZipWriter<File> {
    pub fn new(file: File) -> Self {
        Self {
            zip: Mutex::new(Zip::new(file)),
            write_dir: None,
        }
    }

    pub fn init(
        path: &Path,
        extra_files: &[ExtraFile],
        write_dir: Option<&str>,
    ) -> Self {
        let mut zip = Self::new(File::create(path).unwrap());

        for file in extra_files {
            let (source, target) = match file {
                ExtraFile::Short(source) => {
                    let target = PathBuf::from(source.file_name().unwrap());
                    (source.to_owned(), target)
                },
                ExtraFile::Full { source, target } => (source.to_owned(), target.to_owned()),
            };
            match std::fs::read(&source) {
                Ok(contents) => zip.write(&target, contents).unwrap_or_else(|_| {
                    log::warn!("could not write extra file [{}]", target.display());
                }),
                Err(_) => log::warn!("could not read extra file [{}]", source.display()),
            }
        }

        zip.write_dir = write_dir.map(|s| Path::new(s).to_owned());

        zip
    }
}

impl ZipWriter<Cursor<Vec<u8>>> {
    pub fn new(buffer: Vec<u8>) -> Self {
        Self {
            zip: Mutex::new(Zip::new(Cursor::new(buffer))),
            write_dir: None,
        }
    }
}

impl<W: Write + Seek> ZipWriter<W> {
    pub fn finish(&mut self) -> ZipResult<W> {
        self.zip.get_mut().unwrap().finish()
    }
}

impl<W: Write + Seek + Send + Sync> Writer for ZipWriter<W> {
    fn copy(&mut self, entry: &Path) -> Result<()> {
        if entry.is_file() {
            self.write(entry, std::fs::read(entry)?)?;
        } else if entry.is_dir() {
            let walker = WalkBuilder::new(entry).same_file_system(true).build();
            for entry in walker.into_iter().filter_map(|file| file.ok()) {
                self.write(entry.path(), std::fs::read(entry.path())?)?;
            }
        }

        Ok(())
    }

    fn write(&mut self, name: &Path, contents: Vec<u8>) -> Result<()> {
        let zip = self.zip.get_mut().unwrap();

        zip.start_file({
            let name = name.strip_prefix("./").unwrap_or(name);
            self.write_dir.as_ref().map_or(name.to_owned(), |p| p.join(name))
        }.to_string_lossy(), FileOptions::default())?;

        Ok(zip.write_all(&contents)?)
    }
}
