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
    created: Instant,
}

impl App {
    pub fn new(world: PathBuf, config: Config) -> Self {
        Self { world, config, created: Instant::now() }
    }

    pub fn package_dir(&self, to: &Path) {
        utils::print_start(&self.world);
        self.write_packaged_world(to);
        utils::print_done(to, self.created.elapsed());
    }

    pub fn package_zip(&self, to: &Path) {
        utils::print_start(&self.world);
        let temp = TempDir::new().unwrap();
        self.write_packaged_world(temp.path());
        utils::create_zip(temp.path(), &to).unwrap_or_else(|err| log::error!("{err}"));
        utils::print_done(&to.with_extension("zip"), self.created.elapsed());
    }

    fn write_packaged_world(&self, to: &Path) {
        std::env::set_current_dir(&self.world).expect("could not set working dir");

        let entries = self.create_world_entries();
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

    fn create_world_entries(&self) -> Vec<WorldEntry> {
        let entries = Mutex::new(self.config.resourcepack.as_ref().map_or_else(
            || vec![],
            |path| vec![WorldEntry::ResourcePack(ResourcePackEntry::new(path))],
        ));
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
