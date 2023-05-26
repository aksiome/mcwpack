use std::path::Path;
use std::sync::Mutex;

use anyhow::Result;

use crate::config::Config;
use crate::writers::Writer;

mod datapack;
mod file;
mod level;
mod region;
mod resourcepack;
mod scoreboard;

pub struct FilePackager;
pub struct LevelPackager;
pub struct RegionPackager;
pub struct DatapackPackager;
pub struct ScoreboardPackager;
pub struct ResourcepackPackager;

pub trait Packager: Sync {
    /// Check whether an entry can be packaged.
    fn supports(&self, entry: &Path) -> bool;

    /// Package an entry into the given writter.
    fn package(
        &self,
        entry: &Path,
        config: &Config,
        writer: &Mutex<Box<dyn Writer>>,
    ) -> Result<()>;
}
