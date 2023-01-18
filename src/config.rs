use std::fs;
use std::path::{Path, PathBuf};

use ignore::overrides::{Override, OverrideBuilder};
use serde::{Deserialize, Deserializer};

use crate::utils;

pub const DEFAULT_FILENAME: &str = "mcwpack.yaml";
pub const DEFAULT_CONTENTS: &str = "#### MCWPACK CONFIG FILE ####

# Level name (supports minecraft color codes)
name: null
# Resourcepack directory (or zip archive)
resourcepack: null
# Remove empty chunks from the world
clean_chunks: true
# Reset player data in level.dat
reset_player: true
# Archive all non-archived datapacks
zip_datapacks: true
# Filter accepted entries (using glob patterns)
packaged_entries:
  - \"data/*.dat\"
  - \"datapacks/*\"
  - \"poi/*.mca\"
  - \"region/*.mca\"
  - \"entities/*.mca\"
  - \"icon.png\"
  - \"level.dat\"
";

#[derive(Deserialize, Debug)]
pub struct Config {
    pub name: Option<String>,
    #[serde(deserialize_with="deserialize_path")]
    pub resourcepack: Option<PathBuf>,
    #[serde(default)]
    pub clean_chunks: bool,
    #[serde(default)]
    pub reset_player: bool,
    #[serde(default)]
    pub zip_datapacks: bool,
    #[serde(rename="packaged_entries", deserialize_with="deserialize_override")]
    pub overrides: Override,
}

fn deserialize_path<'de, D>(deserializer: D) -> Result<Option<PathBuf>, D::Error>
where
    D: Deserializer<'de>,
{
    let path: Option<PathBuf> = Deserialize::deserialize(deserializer)?;
    path.map(|path| path.canonicalize().map_err(serde::de::Error::custom)).transpose()
}

fn deserialize_override<'de, D>(deserializer: D) -> Result<Override, D::Error>
where
    D: Deserializer<'de>,
{
    let patterns: Vec<String> = Deserialize::deserialize(deserializer)?;
    let mut overrides = OverrideBuilder::new("./");
    for pattern in patterns {
        overrides.add(&pattern).map_err(serde::de::Error::custom)?;
    }
    overrides.build().map_err(serde::de::Error::custom)
}

impl Config {
    pub fn load(path: &Path) -> Option<Self> {
        std::env::set_current_dir(path.parent().unwrap()).expect("could not set working dir");
        fs::read_to_string(path).map_or_else(|_| {
            log::error!("could not read the config file!");
            Self::create_or_edit(path, DEFAULT_CONTENTS)
        }, |contents| Self::try_parse(&contents).or_else(|| Self::create_or_edit(path, &contents)))
    }

    fn try_parse(contents: &str) -> Option<Self> {
        serde_yaml::from_str(contents).map_err(|err| log::error!("{err}")).ok()
    }

    fn create_or_edit(path: &Path, contents: &str) -> Option<Self> {
        if !utils::confirm("Do you want to edit the config file?") {
            return None;
        }
        edit::edit(contents).ok().and_then(|contents| {
            Self::try_parse(&contents).map(|config| {
                if utils::confirm("Do you want to save the config file?") {
                    fs::write(path, &contents).unwrap_or_else(|err| {
                        log::error!("{err}");
                    });
                }
                config
            }).or_else(|| Self::create_or_edit(path, &contents))
        })
    }
}
