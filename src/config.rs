use std::path::{Path, PathBuf};

use globset::{Glob, GlobSet, GlobSetBuilder};
use ignore::overrides::{Override, OverrideBuilder};
use path_absolutize::Absolutize;
use serde::{Deserialize, Deserializer};

use crate::entries::ExtraEntry;
use crate::utils;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub dirname: Option<PathBuf>,
    #[serde(default, deserialize_with = "deserialize_extra_entries")]
    pub extra_entries: Vec<ExtraEntry>,
    #[serde(default, deserialize_with = "deserialize_resourcepack")]
    pub resourcepack: Option<PathBuf>,
    #[serde(default)]
    pub reset_player: bool,
    #[serde(default)]
    pub zip_datapacks: bool,
    #[serde(default)]
    pub clean_chunks: bool,
    #[serde(default = "ignored_blocks")]
    pub ignored_blocks: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_globset")]
    pub accepted_scores: GlobSet,
    #[serde(default, deserialize_with = "deserialize_globset")]
    pub accepted_objectives: GlobSet,
    #[serde(deserialize_with = "deserialize_override")]
    pub accepted_entries: Override,
}

fn ignored_blocks() -> Vec<String> {
    vec!["minecraft:air".to_owned()]
}

fn deserialize_extra_entries<'de, D>(deserializer: D) -> Result<Vec<ExtraEntry>, D::Error>
where
    D: Deserializer<'de>,
{
    let entries: Vec<ExtraEntry> = Deserialize::deserialize(deserializer)?;
    Ok(entries.iter().filter_map(|entry| {
        entry.canonicalize().map_err(|err| {
            log::warn!("could not read extra entry ({err})")
        }).ok()
    }).collect())
}

fn deserialize_resourcepack<'de, D>(deserializer: D) -> Result<Option<PathBuf>, D::Error>
where
    D: Deserializer<'de>,
{
    let path: Option<PathBuf> = Deserialize::deserialize(deserializer)?;
    Ok(path.and_then(|path| path.canonicalize().map_err(|err| {
        log::warn!("could not read resourcepack ({err})")
    }).ok()))
}

fn deserialize_globset<'de, D>(deserializer: D) -> Result<GlobSet, D::Error>
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
    pub fn load(path: &Path, noprompt: bool) -> Option<Self> {
        let conf_path = path.absolutize().unwrap();
        let current_dir = std::env::current_dir().expect("could not get working dir");
        std::env::set_current_dir(conf_path.parent()?).expect("could not set working dir");
        let config = Self::try_load(&conf_path, noprompt);
        std::env::set_current_dir(current_dir).expect("could not set working dir");
        config
    }

    fn try_load(path: &Path, noprompt: bool) -> Option<Self> {
        std::fs::read_to_string(path).map_or_else(|err| {
            log::error!("could not read the config file ({err})");
            Self::try_edit(path, include_str!("../config.yaml"), noprompt)
        }, |contents| Self::parse(&contents).or_else(|| {
            Self::try_edit(path, &contents, noprompt)
        }))
    }

    fn try_edit(path: &Path, contents: &str, noprompt: bool) -> Option<Self> {
        match noprompt {
            true => None,
            false => Self::edit(path, contents),
        }
    }

    fn parse(contents: &str) -> Option<Self> {
        serde_yaml::from_str(contents).map_err(|err| {
            log::error!("could not parse the config file ({err})");
        }).ok()
    }

    fn edit(path: &Path, contents: &str) -> Option<Self> {
        if !utils::confirm("Do you want to edit the config file?", true) {
            return None;
        }

        edit::edit(contents).map_or_else(|err| {
            log::error!("could not edit the config file ({err})");
            None
        }, |contents| {
            Self::parse(&contents).map(|config| {
                if utils::confirm("Do you want to save the config file?", true) {
                    std::fs::write(path, &contents).unwrap_or_else(|err| {
                        log::error!("could not save the config file ({err})");
                    });
                }
                config
            }).or_else(|| Self::edit(path, &contents))
        })
    }
}
