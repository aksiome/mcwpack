use std::env;
use std::path::PathBuf;

use app::{App, Output};
use clap::{Parser, crate_version};
use config::Config;
use log::LevelFilter;

mod app;
mod config;
mod entries;
mod models;
mod utils;

#[derive(Parser)]
#[clap(name = "Minecraft World Packager", version = crate_version!(), author = "Aksiome")]
pub struct Opts {
    #[clap(value_name = "WORLD_PATH")]
    world: Option<PathBuf>,
    /// Set the output zip
    #[arg(short, value_name = "ZIP_PATH", conflicts_with = "dir")]
    zip: Option<PathBuf>,
    /// Set the output directory
    #[arg(short, value_name = "DIR_PATH", conflicts_with = "zip")]
    dir: Option<PathBuf>,
    /// Use the given config file
    #[arg(short, value_name = "CONFIG_FILE")]
    config: Option<PathBuf>,
    /// Show debug trace
    #[arg(short, conflicts_with = "quiet")]
    verbose: bool,
    /// Silence warning
    #[arg(short, conflicts_with = "verbose")]
    quiet: bool,
}

fn main() {
    let opts = Opts::parse();
    utils::init_logger(
        if opts.verbose { LevelFilter::Trace }
        else if opts.quiet { LevelFilter::Error }
        else { LevelFilter::Warn }
    );

    let root = env::current_dir().unwrap();
    let world = root.join(opts.world.unwrap_or_else(|| utils::enter_path("Please enter the world path: ", true)));
    let config = root.join(opts.config.unwrap_or_else(|| world.join(config::DEFAULT_FILENAME)));

    if let Some(config) = Config::load(&config) {
        let app = App::new(world, config);
        match (opts.dir, opts.zip) {
            (Some(to), _) => app.package(Output::Dir(root.join(to))),
            (_, Some(to)) => app.package(Output::Zip(root.join(to))),
            _ => app.package(Output::Zip(root.join(utils::enter_path("Please enter the zip output path: ", false)))),
        }
    }
}
