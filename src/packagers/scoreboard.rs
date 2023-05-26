use std::path::Path;
use std::sync::Mutex;

use anyhow::Result;

use super::{Packager, ScoreboardPackager};
use crate::config::Config;
use crate::models::nbt::NbtFormat;
use crate::models::scoreboard::Scoreboard;
use crate::writers::Writer;

impl Packager for ScoreboardPackager {
    fn supports(&self, entry: &Path) -> bool {
        entry.is_file() && entry.file_name().map_or(false, |name| name == "scoreboard.dat")
    }

    fn package(
        &self,
        entry: &Path,
        config: &Config,
        writer: &Mutex<Box<dyn Writer>>,
    ) -> Result<()> {
        let mut nbt = Scoreboard::load(entry)?;
        if !config.accepted_scores.is_empty() {
            nbt.data.scores.retain(|e| config.accepted_scores.is_match(&e.name));
        }
        if !config.accepted_objectives.is_empty() {
            nbt.data.objectives.retain(|e| config.accepted_objectives.is_match(&e.name));
        }

        writer.lock().unwrap().write(entry, nbt.to_bytes()?)
    }
}
