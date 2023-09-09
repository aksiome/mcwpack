use crate::formats::{Level, NbtFormat};
use crate::utils::PrefixPath;
use super::*;

#[derive(Clone, Debug, Deref, From)]
pub struct LevelEntry(PathBuf);

impl Packageable for LevelEntry {}

impl<S: Storage> Visitor<LevelEntry> for Packager<S> {
    fn visit(&self, entry: &LevelEntry) -> Result<()> {
        let mut nbt = Level::load(entry)?;

        if let Some(name) = &self.config.name {
            nbt.data.name = name.to_owned();
        }
        if self.config.reset_player {
            nbt.data.player.clear();
        }
        if self.config.zip_datapacks {
            nbt.walk_datapacks(|value: &mut String| {
                if value.starts_with("file/") && !value.ends_with(".zip") {
                    value.push_str(".zip");
                }
            });
        };

        self.target.write(&entry.prefix(self.config.dirname.as_ref()), &nbt.to_bytes()?)
    }
}
