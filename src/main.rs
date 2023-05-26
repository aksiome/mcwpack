use std::path::PathBuf;

use app::{App, Target};
use clap::Parser;
use config::Config;
use log::LevelFilter;
use logger::Logger;

mod app;
mod config;
mod logger;
mod utils;
mod models;
mod packagers;
mod writers;

#[derive(Parser)]
#[clap(
    name = "Minecraft World Packager",
    version = clap::crate_version!(),
    author = "Aksiome",
)]
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

    Logger::init(match opts {
        _ if opts.verbose => LevelFilter::Trace,
        _ if opts.quiet => LevelFilter::Error,
        _ => LevelFilter::Warn,
    });

    let root = std::env::current_dir().unwrap();
    let world = root.join(opts.world.unwrap_or_else(|| utils::enter_path("Please enter the world path: ", true)));
    let config = root.join(opts.config.unwrap_or_else(|| world.join(config::DEFAULT_FILENAME)));

    if let Some(config) = Config::load(&config) {
        let target = match (opts.dir, opts.zip) {
            (Some(path), _) => Target::Dir(root.join(path)),
            (_, Some(path)) => Target::Zip(root.join(path).with_extension("zip")),
            _ => {
                let path = utils::enter_path("Please enter the zip output path: ", false);
                Target::Zip(root.join(path).with_extension("zip"))
            } 
        };

        if match &target {
            Target::Dir(path) if path.exists() => utils::confirm(
                "The output directory already exists, do you want to continue?",
                false,
            ),
            Target::Zip(path) if path.exists() => utils::confirm(
                "The output zip file already exists, do you want to replace it?",
                false,
            ),
            _ => true,
        } {
            App::new(config, world).run(target)
        }
    }
}
