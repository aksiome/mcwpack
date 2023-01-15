use std::path::Path;

use anyhow::Result;
use enum_dispatch::enum_dispatch;

use crate::config::Config;

use self::dp::DataPackEntry;
use self::file::FileEntry;
use self::level::LevelEntry;
use self::region::RegionEntry;
use self::rp::ResourcePackEntry;

pub mod dp;
pub mod file;
pub mod level;
pub mod region;
pub mod rp;

#[enum_dispatch]
pub trait Entry {
    fn package(&self, config: &Config, to: &Path) -> Result<()>;
}

#[enum_dispatch(Entry)]
pub enum WorldEntry {
    ResourcePack(ResourcePackEntry),
    DataPack(DataPackEntry),
    Region(RegionEntry),
    Level(LevelEntry),
    File(FileEntry),
}

impl WorldEntry {
    pub fn guess(path: &Path) -> Option<Self> {
        if let Some(entry) = DataPackEntry::new(path) {
            Some(WorldEntry::from(entry))
        } else if let Some(entry) = RegionEntry::new(path) {
            Some(WorldEntry::from(entry))
        } else if let Some(entry) = LevelEntry::new(path) {
            Some(WorldEntry::from(entry))
        } else if let Some(entry) = FileEntry::new(path) {
            Some(WorldEntry::from(entry))
        } else {
            None
        }
    }
}
