use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Instant;

use ignore::{WalkBuilder, WalkState::*};
use indicatif::{ProgressBar, ProgressDrawTarget, ParallelProgressIterator, ProgressStyle};
use rayon::prelude::*;

use crate::config::Config;
use crate::packagers::*;
use crate::utils;
use crate::writers::Writer;
use crate::writers::dir::DirWriter;
use crate::writers::zip::ZipWriter;

lazy_static::lazy_static! {
    pub static ref PROGRESS: ProgressBar = {
        let template = "   {prefix:.cyan.bold} [{bar:35}] {pos}/{len} files";
        let style = ProgressStyle::with_template(template).unwrap().progress_chars("=>-");
        ProgressBar::hidden().with_style(style).with_prefix("Progress")
    };
}

pub struct App {
    config: Config,
    world: PathBuf,
}

pub enum Target {
    Dir(PathBuf),
    Zip(PathBuf),
}

impl Target {
    pub fn path(&self) -> &Path {
        match self {
            Self::Dir(path) => path,
            Self::Zip(path) => path,
        }
    }

    pub fn writer(&self, app: &App) -> Box<dyn Writer> {
        match self {
            Self::Dir(path) => Box::new(DirWriter::new(path)),
            Self::Zip(path) => Box::new(ZipWriter::init(
                path,
                &app.config.extra_files,
                app.config.dirname.as_deref(),
            )),
        }
    }
}

impl App {
    pub fn new(config: Config, world: PathBuf) -> Self {
        Self { config, world }
    }

    pub fn run(&self, target: Target) {
        std::env::set_current_dir(&self.world).expect("could not set working dir");

        let time = Instant::now();
        utils::print_start(&self.world);

        let mut entries = vec![];
        entries.append(&mut self.world_entries());
        entries.append(&mut self.extra_entries());

        PROGRESS.reset();
        PROGRESS.set_length(entries.len() as u64);
        PROGRESS.set_draw_target(ProgressDrawTarget::stderr());
        self.package(&entries, Mutex::new(target.writer(self)));
        PROGRESS.finish_and_clear();

        utils::print_finish(target.path(), &time.elapsed());
    }

    fn package(&self, entries: &[(PathBuf, &dyn Packager)], writer: Mutex<Box<dyn Writer>>) {
        entries.par_iter().progress_with(PROGRESS.to_owned()).for_each(|(path, packager)| {
            packager.package(path, &self.config, &writer).unwrap_or_else(|err| {
                log::warn!("{err} [{}]", path.display());
            });
        });
    }

    fn extra_entries(&self) -> Vec<(PathBuf, &dyn Packager)> {
        let mut entries = vec![];
        if let Some(path) = &self.config.resourcepack {
            entries.push((path.to_owned(), &ResourcepackPackager as &dyn Packager));
        }
        entries
    }

    fn world_entries(&self) -> Vec<(PathBuf, &dyn Packager)> {
        let entries = Mutex::new(vec![]);
        let walker = WalkBuilder::new("./")
            .overrides(self.config.accepted_entries.to_owned())
            .same_file_system(true)
            .build_parallel();

        walker.run(|| Box::new(|result| {
            match result.ok().and_then(|e| [
                &DatapackPackager as &dyn Packager,
                &RegionPackager as &dyn Packager,
                &ScoreboardPackager as &dyn Packager,
                &LevelPackager as &dyn Packager,
                &FilePackager as &dyn Packager,
            ].iter().find(|p| p.supports(e.path())).map(|p| (e.path().to_owned(), *p))) {
                None => Continue,
                Some(e) => {
                    entries.lock().unwrap().push(e);
                    Skip
                }
            }
        }));

        entries.into_inner().unwrap()
    }
}
