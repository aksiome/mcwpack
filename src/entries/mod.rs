pub use datapack::DatapackEntry;
pub use extra::ExtraEntry;
pub use file::FileEntry;
pub use level::LevelEntry;
pub use region::RegionEntry;
pub use resourcepack::ResourcepackEntry;
pub use scoreboard::ScoreboardEntry;

mod datapack;
mod extra;
mod file;
mod level;
mod region;
mod resourcepack;
mod scoreboard;

use std::path::{Path, PathBuf};

use anyhow::{Result, Context};
use derive_more::{Deref, From};

use crate::Packager;
use crate::storage::Storage;

pub trait Visitor<T> {
    fn visit(&self, entry: &T) -> Result<()>;
}

pub trait Packageable: Sized {
    fn package<V: Visitor<Self>>(&self, visitor: &V) -> Result<()> {
        visitor.visit(self)
    }
}

pub enum Entry {
    Datapack(self::datapack::DatapackEntry),
    Extra(self::extra::ExtraEntry),
    File(self::file::FileEntry),
    Level(self::level::LevelEntry),
    Region(self::region::RegionEntry),
    Resourcepack(self::resourcepack::ResourcepackEntry),
    Scoreboard(self::scoreboard::ScoreboardEntry),
}

impl Entry {
    pub fn path(&self) -> &Path {
        match self {
            Self::Datapack(entry) => entry,
            Self::Extra(entry) => entry,
            Self::File(entry) => entry,
            Self::Level(entry) => entry,
            Self::Region(entry) => entry,
            Self::Resourcepack(entry) => entry,
            Self::Scoreboard(entry) => entry,
        }
    }

    pub fn guess(path: &Path) -> Option<Self> {
        if path.starts_with("./datapacks") && path.components().count() > 2 {
            Some(Entry::Datapack(path.to_owned().into()))
        } else if !path.is_file() {
            None
        } else if path.extension().map_or(false, |ext| ext == "mca") {
            Some(Entry::Region(path.to_owned().into()))
        } else if path.file_name().map_or(false, |name| name == "scoreboard.dat") {
            Some(Entry::Scoreboard(path.to_owned().into()))
        } else if path.file_name().map_or(false, |name| name == "level.dat") {
            Some(Entry::Level(path.to_owned().into()))
        } else {
            Some(Entry::File(path.to_owned().into()))
        }
    }
}

impl Packageable for Entry {}

impl<S: Storage> Visitor<Entry> for Packager<S> {
    fn visit(&self, entry: &Entry) -> Result<()> {
        match entry {
            Entry::Datapack(entry) => entry.package(self),
            Entry::Extra(entry) => entry.package(self),
            Entry::File(entry) => entry.package(self),
            Entry::Level(entry) => entry.package(self),
            Entry::Region(entry) => entry.package(self),
            Entry::Resourcepack(entry) => entry.package(self),
            Entry::Scoreboard(entry) => entry.package(self),
        }
    }
}
