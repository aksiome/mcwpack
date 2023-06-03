use std::path::{Path, PathBuf};

use globset::{Glob, GlobSet, GlobSetBuilder};
use ignore::overrides::{Override, OverrideBuilder};
use serde::{Deserialize, Deserializer};

use crate::utils;

pub const DEFAULT_FILENAME: &str = "mcwpack.yaml";
pub const DEFAULT_CONTENTS: &str = "#### MCWPACK CONFIG FILE ####

# Level name (supports minecraft color codes)
name: null
# Directory name inside the archive
dirname: null
# Resourcepack directory (or zip archive)
resourcepack: null
# Reset player data in level.dat
reset_player: true
# Archive all non-archived datapacks
zip_datapacks: true
# Remove empty chunks from the world
clean_chunks: true
# Ignored blocks when cleaning chunks (default: minecraft:air)
# The following will delete chunks that only contain air or stone
# ignored_blocks:
#   - minecraft:air
#   - minecraft:stone
# Filter accepted scores (using glob patterns)
# accepted_scores:
# Filter accepted objectives (using glob patterns)
# accepted_objectives:
# Filter accepted file entries (using glob patterns)
accepted_entries:
  - data/*.dat
  - datapacks/*
  - poi/*.mca
  - region/*.mca
  - entities/*.mca
  - icon.png
  - level.dat
";

#[derive(Deserialize, Debug)]
pub struct Config {
    pub name: Option<String>,
    pub dirname: Option<String>,
    #[serde(deserialize_with = "deserialize_path")]
    pub resourcepack: Option<PathBuf>,
    #[serde(default)]
    pub reset_player: bool,
    #[serde(default)]
    pub zip_datapacks: bool,
    #[serde(default)]
    pub clean_chunks: bool,
    #[serde(default = "ignored_blocks")]
    pub ignored_blocks: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_globs")]
    pub accepted_scores: GlobSet,
    #[serde(default, deserialize_with = "deserialize_globs")]
    pub accepted_objectives: GlobSet,
    #[serde(deserialize_with = "deserialize_override")]
    pub accepted_entries: Override,
}

fn ignored_blocks() -> Vec<String> {
    vec!["minecraft:air".to_owned()]
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

fn deserialize_globs<'de, D>(deserializer: D) -> Result<GlobSet, D::Error>
where
    D: Deserializer<'de>,
{
    let patterns: Vec<String> = Deserialize::deserialize(deserializer)?;
    let mut globset = GlobSetBuilder::new();
    for pattern in patterns {
        globset.add(Glob::new(&pattern).map_err(serde::de::Error::custom)?);
    }
    globset.build().map_err(serde::de::Error::custom)
}

impl Config {
    pub fn load(path: &Path) -> Option<Self> {
        std::env::set_current_dir(path.parent().unwrap()).expect("could not set working dir");
        std::fs::read_to_string(path).map_or_else(|_| {
            log::error!("could not read the config file!");
            Self::create_or_edit(path, DEFAULT_CONTENTS)
        }, |contents| Self::try_parse(&contents).or_else(|| Self::create_or_edit(path, &contents)))
    }

    fn try_parse(contents: &str) -> Option<Self> {
        serde_yaml::from_str(contents).map_err(|err| log::error!("{err}")).ok()
    }

    fn create_or_edit(path: &Path, contents: &str) -> Option<Self> {
        if !utils::confirm("Do you want to edit the config file?", true) {
            return None;
        }
        edit::edit(contents).ok().and_then(|contents| {
            Self::try_parse(&contents).map(|config| {
                if utils::confirm("Do you want to save the config file?", true) {
                    std::fs::write(path, &contents).unwrap_or_else(|err| {
                        log::error!("{err}");
                    });
                }
                config
            }).or_else(|| Self::create_or_edit(path, &contents))
        })
    }
}
