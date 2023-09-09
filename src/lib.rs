pub mod entries;
pub mod formats;
pub mod storage;
pub mod utils;

pub use config::Config;

mod config;

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Instant;

use entries::{Entry, Packageable};
use ignore::{WalkBuilder, WalkState};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use storage::{DirStorage, FilesystemStorage, Storage, ZipStorage};

const PB_SPACING: &str = "   ";
const PB_TEMPLATE: &str = "{prefix:.cyan.bold} [{bar:35}] {pos}/{len} files";

pub enum Context {
    Zip,
    Dir,
}

impl Context {
    pub fn package(&self, config: Config, from: &Path, to: &Path) {
        match self {
            Self::Dir => Packager::new(config, from.to_owned(), DirStorage::new(to)).run(),
            Self::Zip => Packager::new(config, from.to_owned(), ZipStorage::new(to)).run(),
        }
    }
}

pub struct Packager<S: Storage> {
    config: Config,
    world: PathBuf,
    target: S,
    progress: ProgressBar,
}

impl<S: Storage> Packager<S> {
    pub fn new(config: Config, world: PathBuf, target: S) -> Self {
        let tmpl = format!("{}{}", PB_SPACING, PB_TEMPLATE);
        let style = ProgressStyle::with_template(&tmpl).unwrap().progress_chars("=>-");
        let progress = ProgressBar::new(0).with_style(style).with_prefix("Progress");

        Self { config, world, target, progress }
    }

    pub fn run(&self) {
        std::env::set_current_dir(&self.world).expect("could not set working dir");

        let mut entries = vec![];
        entries.extend(self.world_entries());
        entries.extend(self.extra_entries());

        self.package(&entries)
    }

    pub fn package(&self, entries: &[Entry]) {
        let time = Instant::now();
        utils::print_start(&self.world);
        self.progress.set_length(entries.len() as u64);

        entries.par_iter().for_each(|entry| {
            entry.package(self).unwrap_or_else(|err| self.progress.suspend(|| {
                log::warn!("{}{err} [{}]", PB_SPACING, entry.path().display())
            }));
        });

        utils::print_finish(self.target.path().unwrap_or(&self.world), &time.elapsed());
    }

    pub fn extra_entries(&self) -> Vec<Entry> {
        let mut entries = vec![];
        entries.extend(self.config.extra_entries.iter().map(|e| Entry::Extra(e.to_owned())));
        entries.extend(self.config.resourcepack.to_owned().map(|e| Entry::Resourcepack(e.into())));

        entries
    }

    pub fn world_entries(&self) -> Vec<Entry> {
        let entries = Mutex::new(vec![]);
        let walker = WalkBuilder::new("./")
            .overrides(self.config.accepted_entries.to_owned())
            .same_file_system(true)
            .build_parallel();

        walker.run(|| Box::new(|result| {
            match result.ok().and_then(|e| Entry::guess(e.path())) {
                None => WalkState::Continue,
                Some(entry) => {
                    entries.lock().unwrap().push(entry);
                    WalkState::Skip
                },
            }
        }));

        entries.into_inner().unwrap()
    }
}
