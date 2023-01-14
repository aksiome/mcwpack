use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::Result;
use dialoguer::Confirm;
use fs_extra::dir;
use zip_extensions::zip_create_from_directory;

pub fn confirm(message: &str) -> bool {
    Confirm::new()
        .with_prompt(message)
        .interact()
        .unwrap_or(false)
}

pub fn with_extension(path: &Path, ext: &str) -> PathBuf {
    let mut path = path.to_owned();
    path.set_extension(ext);
    path
}

pub fn copy_dir(from: &Path, to: &Path) -> Result<()> {
    let mut options = dir::CopyOptions::new();
    options.content_only = true;
    dir::copy(from, to, &options)?;
    Ok(())
}

pub fn create_zip(from: &Path, to: &Path) -> Result<()> {
    let to = with_extension(to, "zip");
    zip_create_from_directory(&to, &from.to_owned())?;
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
