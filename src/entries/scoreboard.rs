use std::fs;
use std::path::{PathBuf, Path};

use anyhow::Result;

use crate::config::Config;
use crate::models::scoreboard::Scoreboard;

use super::Entry;
use super::WorldEntry;

pub struct ScoreboardEntry {
    path: PathBuf,
}

impl ScoreboardEntry {
    pub fn try_create(path: &Path) -> Option<WorldEntry> {
        (path.is_file() && path.file_name().map_or(false, |name| name == "scoreboard.dat")).then(
            || WorldEntry::Scoreboard(Self { path: path.to_owned() })
        )
    }
}

impl Entry for ScoreboardEntry {
    fn package(&self, config: &Config, to: &Path) -> Result<()> {
        log::info!("processing scoreboard ({})", self.path.to_string_lossy());
        let to = to.to_owned().join(&self.path);
        fs::create_dir_all(to.parent().unwrap())?;
        let mut level = Scoreboard::load(&self.path)?;

        if !config.accepted_scores.is_empty() {
            level.data.scores.retain(|e| config.accepted_scores.is_match(&e.name));
        }
        if !config.accepted_objectives.is_empty() {
            level.data.objectives.retain(|e| config.accepted_objectives.is_match(&e.name));
        }

        level.write(&to)
    }
}
