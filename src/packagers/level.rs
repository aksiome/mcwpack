use std::path::Path;
use std::sync::Mutex;

use anyhow::Result;

use super::{LevelPackager, Packager};
use crate::config::Config;
use crate::models::level::Level;
use crate::models::nbt::NbtFormat;
use crate::writers::Writer;

impl Packager for LevelPackager {
    fn supports(&self, entry: &Path) -> bool {
        entry.is_file() && entry.file_name().map_or(false, |name| name == "level.dat")
    }

    fn package(
        &self,
        entry: &Path,
        config: &Config,
        writer: &Mutex<Box<dyn Writer>>,
    ) -> Result<()> {
        let mut nbt = Level::load(entry)?;
        if let Some(name) = &config.name {
            nbt.data.name = name.to_owned()
        }
        if config.reset_player {
            nbt.data.player.clear()
        }
        if config.zip_datapacks {
            nbt.walk_datapacks(|value: &mut String| {
                if value.starts_with("file/") && !value.ends_with(".zip") {
                    value.push_str(".zip");
                }
            })
        };

        writer.lock().unwrap().write(entry, nbt.to_bytes()?)
    }
}
