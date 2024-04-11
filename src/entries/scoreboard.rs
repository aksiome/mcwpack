use crate::formats::{NbtFormat, Scoreboard};
use crate::utils::PathUtils;
use super::*;

#[derive(Clone, Debug, Deref, From)]
pub struct ScoreboardEntry(PathBuf);

impl Packageable for ScoreboardEntry {}

impl<S: Storage> Visitor<ScoreboardEntry> for Packager<S> {
    fn visit(&self, entry: &ScoreboardEntry) -> Result<()> {
        let mut nbt = Scoreboard::load(entry)?;

        if !self.config.accepted_scores.is_empty() {
            nbt.data.scores.retain(|e| {
                self.config.accepted_scores.is_match(&e.name)
            });
        }
        if !self.config.accepted_objectives.is_empty() {
            nbt.data.objectives.retain(|e| {
                self.config.accepted_objectives.is_match(&e.name)
            });
        }

        self.target.write(&entry.prefix(self.config.dirname.as_ref()), &nbt.to_bytes()?)
    }
}
