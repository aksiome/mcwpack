use std::fs;
use std::path::{PathBuf, Path};

use anyhow::Result;

use crate::config::Config;
use crate::entries::Entry;
use crate::models::level::Level;

pub struct LevelEntry {
    path: PathBuf,
}

impl LevelEntry {
    pub fn new(path: &Path) -> Option<Self> {
        (path.is_file() && path.file_name().map_or(false, |name| name == "level.dat")).then(
            || Self { path: path.to_owned() }
        )
    }
}

impl Entry for LevelEntry {
    fn package(&self, config: &Config, to: &Path) -> Result<()> {
        let to = to.to_owned().join(&self.path);
        fs::create_dir_all(&to.parent().unwrap())?;
        let mut level = Level::load(&self.path)?;
        config.name.as_ref().map(|name| level.set_name(&name));
        config.reset_player.then(|| level.reset_player());
        config.zip_datapacks.then(|| level.update_all_datapacks(|value: &mut String| {
            if value.starts_with("file/") && !value.ends_with(".zip") {
                value.push_str(".zip");
            } 
        }));
        level.write(&to)
    }
}
