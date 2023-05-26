use std::path::Path;
use std::sync::Mutex;

use anyhow::{Context, Result};

use super::{Packager, RegionPackager};
use crate::config::Config;
use crate::models::region::Region;
use crate::writers::Writer;

impl Packager for RegionPackager {
    fn supports(&self, entry: &Path) -> bool {
        entry.is_file() && entry.extension().map_or(false, |ext| ext == "mca")
    }

    fn package(
        &self,
        entry: &Path,
        config: &Config,
        writer: &Mutex<Box<dyn Writer>>,
    ) -> Result<()> {
        if entry.metadata()?.len() <= 8192 {
            log::info!("skipped empty region: {}", entry.to_string_lossy());
            return Ok(());
        }

        match config.clean_chunks {
            true => {
                let contents = Region::load(entry).with_context(
                    || format!("could not read region: {}", entry.to_string_lossy())
                )?.optimize(config).with_context(
                    || format!("could not process region: {}", entry.to_string_lossy())
                )?;
                match contents.len() {
                    ..=8192 => log::info!("skipped empty region: {}", entry.to_string_lossy()),
                    _ => writer.lock().unwrap().write(entry, contents)?,
                }
            },
            false => writer.lock().unwrap().copy(entry)?,
        };

        Ok(())
    }
}
