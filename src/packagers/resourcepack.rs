use std::path::Path;
use std::sync::Mutex;

use anyhow::{Result, Context};

use super::{Packager, ResourcepackPackager};
use crate::config::Config;
use crate::utils;
use crate::writers::Writer;

impl Packager for ResourcepackPackager {
    fn supports(&self, entry: &Path) -> bool {
        entry.exists()
    }

    fn package(
        &self,
        entry: &Path,
        _: &Config,
        writer: &Mutex<Box<dyn Writer>>,
    ) -> Result<()> {
        match entry.is_file() {
            true => writer.lock().unwrap().write(
                Path::new("resources.zip"),
                std::fs::read(entry).with_context(|| "could not read resourcepack")?
            ),
            false => writer.lock().unwrap().write(
                Path::new("resources.zip"),
                utils::create_zip_from_dir(entry).with_context(|| "could not read resourcepack")?
            ),
        }
    }
}
