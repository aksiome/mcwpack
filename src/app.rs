use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Instant;

use ignore::WalkBuilder;
use ignore::WalkState::*;
use indicatif::{ProgressStyle, ProgressBar, ParallelProgressIterator};
use tempfile::TempDir;
use rayon::prelude::*;

use crate::config::Config;
use crate::entries::rp::ResourcePackEntry;
use crate::entries::{WorldEntry, Entry};
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
        let started = Instant::now();
        utils::print_start(&self.world);
        utils::print_done(&match output {
            Output::Dir(to) => {
                self.package_to(&to);
                to
            }
            Output::Zip(to) => {
                let temp = TempDir::new().unwrap();
                self.package_to(temp.path());
                utils::create_zip(temp.path(), &to).unwrap_or_else(|err| log::error!("{err}"));
                to.with_extension("zip")
            }
        }, started.elapsed());
    }

    fn package_to(&self, to: &Path) {
        std::env::set_current_dir(&self.world).expect("could not set working dir");

        let mut entries = Vec::new();
        entries.append(&mut self.extra_entries());
        entries.append(&mut self.walker_entries());

        let progress = ProgressBar::new(entries.len() as u64)
            .with_style(ProgressStyle::with_template(PROGRESS).unwrap().progress_chars("=>-"))
            .with_prefix("Progress");

        entries.par_iter().progress_with(progress.to_owned()).for_each(|entry| {
            let res = entry.package(&self.config, to);
            res.unwrap_or_else(|err| progress.suspend(|| {
                log::warn!("{err}");
                for cause in err.chain().skip(1) {
                    log::trace!("{cause}");
                }
            }));
        });
    }

    fn extra_entries(&self) -> Vec<WorldEntry> {
        let mut entries = Vec::new();
        if let Some(path) = &self.config.resourcepack {
            entries.push(WorldEntry::ResourcePack(ResourcePackEntry::new(&path)))
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
            if let Some(entry) = entry.ok().and_then(|entry| WorldEntry::guess(entry.path())) {
                entries.lock().unwrap().push(entry);
                return Skip;
            }
            Continue
        }));
        entries.into_inner().unwrap()
    }
}
