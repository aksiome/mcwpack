use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Instant;

use ignore::{WalkBuilder, WalkState::*};
use indicatif::ProgressBar;
use indicatif::ProgressDrawTarget;
use indicatif::ParallelProgressIterator;
use indicatif::ProgressStyle;
use rayon::prelude::*;

use crate::config::Config;
use crate::packagers::*;
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

pub enum Target {
    Dir(PathBuf),
    Zip(PathBuf),
}

pub struct App {
    config: Config,
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
            Self::Zip(path) => Box::new(ZipWriter::create(
                path,
                app.config.dirname.as_deref()
            )),
        }
    }
}

impl App {
    pub fn new(config: Config, world: PathBuf) -> Self {
        std::env::set_current_dir(world).expect("could not set working dir");

        Self { config }
    }

    pub fn run(&self, target: Target) {
        println!(
            "  {} {} ({})",
            console::style("Packaging").green().bold(),
            std::env::current_dir().unwrap().file_name().unwrap().to_string_lossy(),
            std::env::current_dir().unwrap().to_string_lossy(),
        );

        let time = Instant::now();
        let mut entries = vec![];
        entries.append(&mut self.world_entries());
        entries.append(&mut self.extra_entries());
        self.package(entries, Mutex::new(target.writer(&self)));

        println!(
            "   {} {} ({}) in {:.2}s",
            console::style("Finished").green().bold(),
            target.path().file_name().unwrap().to_string_lossy(),
            target.path().canonicalize().unwrap().to_string_lossy(),
            time.elapsed().as_secs_f32(),
        );
    }

    fn package(&self, entries: Vec<(PathBuf, &dyn Packager)>, writer: Mutex<Box<dyn Writer>>) {
        PROGRESS.set_length(entries.len() as u64);
        PROGRESS.set_draw_target(ProgressDrawTarget::stderr());

        entries.par_iter().progress_with(PROGRESS.to_owned()).for_each(|(path, packager)| {
            packager.package(path, &self.config, &writer).unwrap_or_else(|err| {
                log::warn!("{err}");
            });
        });

        PROGRESS.finish_and_clear();
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
