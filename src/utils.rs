use std::fs;
use std::io::Write;
use std::path::{self, Path, PathBuf};
use std::str::FromStr;
use std::time::Duration;

use anyhow::Result;
use env_logger::Builder;
use env_logger::fmt::Color;
use fs_extra::dir;
use inquire::autocompletion::Replacement;
use inquire::validator::{StringValidator, Validation};
use inquire::{Confirm, Text, CustomUserError, Autocomplete};
use log::{Level, LevelFilter};

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

#[derive(Clone, Default)]
struct PathCompletion {
    input: String,
    values: Vec<String>,
}

impl PathCompletion {
    fn update_values(&mut self, input: &str) -> Result<(), CustomUserError> {
        self.values.clear();
        self.input = input.to_owned();

        let path = PathBuf::from(input);
        let dir = match input.chars().last() {
            Some(c) if path::is_separator(c) => path,
            _ => path.parent().map_or_else(|| PathBuf::from(""), |path| path.to_owned()),
        };

        fs::read_dir(dir)?.filter_map(|entry| entry.ok()).for_each(|entry| {
            let value = match entry.path() {
                path if path.is_dir() => format!("{}{}", path.to_string_lossy(), path::MAIN_SEPARATOR),
                path => path.to_string_lossy().to_string()
            };

            if value.starts_with(&self.input) && value.len() != self.input.len() {
                self.values.push(value);
            }
        });
        Ok(())
    }
}

impl Autocomplete for PathCompletion {
    fn get_completion(&mut self, input: &str, suggestion: Option<String>) -> Result<Replacement, CustomUserError> {
        if input != self.input { self.update_values(input)? }
        Ok(match suggestion {
            Some(suggestion) => Replacement::Some(suggestion),
            None => self.values.first().map(|v| v.to_owned()),
        })
    }

    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        if input != self.input { self.update_values(input)? }
        Ok(self.values.to_owned())
    }
}

pub fn confirm(message: &str) -> bool {
    Confirm::new(message).with_default(true).prompt().unwrap_or(false)
}

pub fn enter_path(message: &str, exists: bool) -> PathBuf {
    if exists {
        Text::new(message).with_autocomplete(PathCompletion::default())
    } else {
        Text::new(message)
    }.with_validator(PathValidator::new(exists)).prompt().map(
        |value| PathBuf::from_str(&value).ok()
    ).ok().flatten().unwrap_or_else(|| enter_path(message, exists))
}

pub fn copy_file(from: &Path, to: &Path) -> Result<()> {
    fs::copy(from, to)?;
    Ok(())
}

pub fn copy_dir(from: &Path, to: &Path) -> Result<()> {
    let mut options = dir::CopyOptions::new();
    options.content_only = true;
    dir::copy(from, to, &options)?;
    Ok(())
}

pub fn create_zip(from: &Path, to: &Path) -> Result<()> {
    zip_extensions::zip_create_from_directory(&to.with_extension("zip"), &from.to_owned())?;
    Ok(())
}

pub fn print_start(from: &Path) {
    println!(
        "  {} {} ({})",
        console::style("Packaging").green().bold(),
        from.file_name().unwrap().to_string_lossy(),
        from.to_string_lossy(),
    );
}

pub fn print_done(to: &Path, time: Duration) {
    println!(
        "   {} {} ({}) in {:.2}s",
        console::style("Finished").green().bold(),
        to.file_name().unwrap().to_string_lossy(),
        to.canonicalize().unwrap().to_string_lossy(),
        time.as_secs_f32(),
    );
}

pub fn init_logger(level: LevelFilter) {
    Builder::new().filter(Some("mcwpack"), level).format(|buf, record| {
        let mut style = buf.style();
        style.set_bold(true);
        writeln!(
            buf,
            "{}: {}",
            match record.level() {
                Level::Trace => style.set_color(Color::Black).value(" ❱"),
                Level::Debug => style.set_color(Color::Magenta).value("debug"),
                Level::Info => style.set_color(Color::Cyan).value("info"),
                Level::Warn => style.set_color(Color::Yellow).value("warn"),
                Level::Error => style.set_color(Color::Red).value("error"),
            },
            record.args(),
        )
    }).init();
}
