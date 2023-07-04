use std::io::Cursor;
use std::path::{PathBuf, Path};
use std::str::FromStr;
use std::time::Duration;

use anyhow::Result;
use ignore::WalkBuilder;
use inquire::validator::{StringValidator, Validation};
use inquire::{Confirm, CustomUserError, Text};

use crate::writers::Writer;
use crate::writers::zip::ZipWriter;

#[derive(Clone)]
struct PathValidator {
    exists: bool,
}

impl PathValidator {
    fn new(exists: bool) -> Self {
        Self { exists }
    }
}

impl StringValidator for PathValidator {
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

pub fn print_start(world: &Path) {
    println!(
        "  {} {} ({})",
        console::style("Packaging").green().bold(),
        world.file_name().unwrap().to_string_lossy(),
        world.to_string_lossy(),
    );
}

pub fn print_finish(target: &Path, duration: &Duration) {
    println!(
        "   {} {} ({}) in {:.2}s",
        console::style("Finished").green().bold(),
        target.file_name().unwrap().to_string_lossy(),
        target.canonicalize().unwrap().to_string_lossy(),
        duration.as_secs_f32(),
    );
}

pub fn confirm(message: &str, default: bool) -> bool {
    Confirm::new(message).with_default(default).prompt().unwrap_or(default)
}

pub fn enter_path(message: &str, exists: bool) -> PathBuf {
    Text::new(message).with_validator(PathValidator::new(exists)).prompt().map(
        |value| PathBuf::from_str(&value).ok()
    ).ok().flatten().unwrap_or_else(|| enter_path(message, exists))
}

pub fn copy_to_dir(entry: &Path, dir: &Path) -> Result<()> {
    let path = dir.join(entry);
    std::fs::create_dir_all(path.parent().unwrap())?;
    std::fs::copy(entry, path)?;

    Ok(())
}

pub fn create_zip_from_dir(dir: &Path) -> Result<Vec<u8>> {
    let mut writer = ZipWriter::<Cursor<Vec<u8>>>::new(vec![]);
    for entry in WalkBuilder::new(dir).same_file_system(true).build() {
        let entry = entry?;
        if entry.path().is_file() {
            writer.write(
                entry.path().strip_prefix(dir)?,
                std::fs::read(entry.path())?,
            )?;
        }
    }

    Ok(writer.finish()?.into_inner())
}
