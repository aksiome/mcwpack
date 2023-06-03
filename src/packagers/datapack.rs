use std::path::Path;
use std::sync::Mutex;

use anyhow::Result;

use super::{DatapackPackager, Packager};
use crate::config::Config;
use crate::utils;
use crate::writers::Writer;

impl Packager for DatapackPackager {
    fn supports(&self, entry: &Path) -> bool {
        entry.starts_with("./datapacks") && entry.components().count() > 2
    }

    fn package(
        &self,
        entry: &Path,
        config: &Config,
        writer: &Mutex<Box<dyn Writer>>,
    ) -> Result<()> {
        match config.zip_datapacks {
            true if entry.is_dir() => writer.lock().unwrap().write(
                &entry.with_extension("zip"),
                utils::create_zip_from_dir(entry)?,
            ),
            _ => writer.lock().unwrap().copy(entry),
        }
    }
}
