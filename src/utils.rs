use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Duration;

use anyhow::Result;
use ignore::WalkBuilder;
use inquire::validator::{StringValidator, Validation};
use inquire::{Confirm, CustomUserError, Text};

use crate::storage::{InMemoryStorage, Storage, ZipStorage};

pub trait PrefixPath {
    fn prefix<P: AsRef<Path>>(&self, prefix: Option<P>) -> PathBuf;
}

impl PrefixPath for PathBuf {
    fn prefix<P: AsRef<Path>>(&self, prefix: Option<P>) -> PathBuf {
        prefix.map_or_else(|| self.to_owned(), |p| p.as_ref().join(self))
    }
}

#[derive(Clone)]
struct StringPathValidator {
    exists: bool,
}

impl StringValidator for StringPathValidator {
    fn validate(&self, input: &str) -> Result<Validation, CustomUserError> {
        Ok(if input.is_empty() {
            Validation::Invalid("A non empty path is required".into())
        } else if self.exists && !PathBuf::from(input).exists() {
            Validation::Invalid("An existing path is required".into())
        } else {
            Validation::Valid
        })
    }
}

pub fn print_start(path: &Path) {
    println!(
        "  {} {} ({})",
        console::style("Packaging").green().bold(),
        path.file_name().unwrap().to_string_lossy(),
        path.display(),
    );
}

pub fn print_finish(path: &Path, duration: &Duration) {
    println!(
        "   {} {} ({}) in {:.2}s",
        console::style("Finished").green().bold(),
        path.file_name().unwrap().to_string_lossy(),
        path.display(),
        duration.as_secs_f32(),
    );
}

pub fn confirm(message: &str, default: bool) -> bool {
    Confirm::new(message)
        .with_default(default)
        .prompt()
        .unwrap_or(default)
}

pub fn enter_path(message: &str, exists: bool) -> PathBuf {
    Text::new(message)
        .with_validator(StringPathValidator { exists })
        .prompt()
        .map(|value| PathBuf::from_str(&value).ok())
        .ok()
        .flatten()
        .unwrap_or_else(|| enter_path(message, exists))
}

pub fn create_zip_from_dir(dir: &Path) -> Result<Vec<u8>> {
    let mut storage = ZipStorage::new(&[]);
    for entry in WalkBuilder::new(dir).same_file_system(true).build() {
        let entry = entry?;
        if entry.path().is_file() {
            storage.write(
                entry.path().strip_prefix(dir)?,
                &std::fs::read(entry.path())?,
            )?;
        }
    }

    Ok(storage.finish()?.into_inner())
}
