use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Instant;

use ignore::WalkBuilder;
use ignore::WalkState::*;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use tempfile::TempDir;

use crate::config::Config;
use crate::entries::dp::DataPackEntry;
use crate::entries::file::FileEntry;
use crate::entries::level::LevelEntry;
use crate::entries::region::RegionEntry;
use crate::entries::rp::ResourcePackEntry;
use crate::entries::{Entry, WorldEntry};
use crate::utils;

const PROGRESS: &str = "   {prefix:.cyan.bold} {bar:35} {pos}/{len} files";

pub struct App {
    world: PathBuf,
    config: Config,
}

pub enum Output {
    Dir(PathBuf),
    Zip(PathBuf),
}

impl App {
    pub fn new(world: PathBuf, config: Config) -> Self {
        Self { world, config }
    }

    pub fn package(&self, output: Output) {
        if match &output {
            Output::Dir(to) if to.exists() =>
                utils::confirm("The output directory already exists, do you want to continue?", false),
            Output::Zip(to) if to.with_extension("zip").exists() =>
                utils::confirm("The output zip file already exists, do you want to replace it?", false),
            _ => true,
        } {
            let started = Instant::now();
            utils::print_start(&self.world);
            utils::print_done(&match output {
                Output::Dir(to) => { self.package_dir(&to); to },
                Output::Zip(to) => { self.package_zip(&to); to.with_extension("zip") },
            }, started.elapsed());
        }
    }

    fn package_dir(&self, to: &Path) {
        std::env::set_current_dir(&self.world).expect("could not set working dir");

        let mut entries = Vec::new();
        entries.append(&mut self.extra_entries());
        entries.append(&mut self.walker_entries());

        let style = ProgressStyle::with_template(PROGRESS).unwrap().progress_chars("=>-");
        let progress = ProgressBar::new(entries.len() as u64).with_style(style).with_prefix("Progress");

        entries.par_iter().progress_with(progress.to_owned()).for_each(|entry| {
            let result = entry.package(&self.config, to);
            result.unwrap_or_else(|err| {
                progress.suspend(|| {
                    log::warn!("{err}");
                    for cause in err.chain().skip(1) {
                        log::trace!("{cause}");
                    }
                })
            });
        });
    }

    fn package_zip(&self, to: &Path) {
        let temp = TempDir::new().unwrap();
        let dirname = self.config.dirname.to_owned().unwrap_or_else(|| {
            self.world.file_name().unwrap().to_string_lossy().to_string()
        });
        self.package_dir(&temp.path().to_owned().join(PathBuf::from(dirname)));
        utils::create_zip(temp.path(), to).unwrap_or_else(|err| log::error!("{err}"));
    }

    fn extra_entries(&self) -> Vec<WorldEntry> {
        let mut entries = Vec::new();
        if let Some(path) = &self.config.resourcepack {
            entries.push(ResourcePackEntry::create(path))
        }
        entries
    }

    fn walker_entries(&self) -> Vec<WorldEntry> {
        let entries = Mutex::new(Vec::new());
        let walker = WalkBuilder::new("./")
            .overrides(self.config.overrides.to_owned())
            .same_file_system(true)
            .build_parallel();

        walker.run(|| Box::new(|entry| {
            let entry = entry.ok().map(|entry| {
                DataPackEntry::try_create(entry.path())
                    .or_else(|| RegionEntry::try_create(entry.path()))
                    .or_else(|| LevelEntry::try_create(entry.path()))
                    .or_else(|| FileEntry::try_create(entry.path()))
            });
            if let Some(entry) = entry.flatten() {
                entries.lock().unwrap().push(entry);
                return Skip;
            }
            Continue
        }));
        entries.into_inner().unwrap()
    }
}
