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
            log::info!("skipped empty region [{}]", entry.display());
            return Ok(());
        }

        match config.clean_chunks {
            true => {
                let contents = Region::load(entry)
                    .with_context(|| "could not read region")?
                    .optimize_bytes(config)
                    .with_context(|| "could not process region")?;

                match contents.len() {
                    ..=8192 => log::info!("skipped empty region [{}]", entry.display()),
                    _ => writer.lock().unwrap().write(entry, contents)?,
                }
            },
            false => writer.lock().unwrap().copy(entry)?,
        };

        Ok(())
    }
}
