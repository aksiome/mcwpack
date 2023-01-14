use std::env;
use std::io::Write;
use std::path::PathBuf;

use app::App;
use clap::Parser;
use config::Config;
use env_logger::Builder;
use env_logger::fmt::Color;
use log::{Level, LevelFilter};

mod app;
mod config;
mod entries;
mod models;
mod utils;

#[derive(Parser)]
#[clap(name = "Minecraft World Packager", version = "0.1", author = "Aksiome")]
pub struct Opts {
    #[clap(value_name = "WORLD_PATH")]
    world: PathBuf,
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
    /// Silences warning
    #[arg(short, conflicts_with = "verbose")]
    quiet: bool,
}

fn main() {
    let opts = Opts::parse();
    init_logger(match (opts.verbose, opts.quiet) {
        (true, false) => LevelFilter::Trace,
        (false, true) => LevelFilter::Error,
        _ => LevelFilter::Warn,
    });
    let root = env::current_dir().unwrap();
    let config = opts.config.unwrap_or_else(
        || opts.world.join(config::DEFAULT_FILENAME)
    );

    if let Some(config) = Config::load(&config) {
        let app = App::new(opts.world, config);
        match (opts.dir, opts.zip) {
            (Some(to), _) => app.package_dir(&root.join(to)),
            (_, Some(to)) => app.package_zip(&root.join(to)),
            _ => todo!(),
        }
    }
}

fn init_logger(level: LevelFilter) {
    Builder::new().filter(Some("mcwpack"), level).format(|buf, record| {
        let mut style = buf.style();
        style.set_bold(true);
        writeln!(
            buf,
            "{} {}",
            match record.level() {
                Level::Trace => style.set_color(Color::Black).value(" ❱"),
                Level::Debug => style.set_color(Color::Magenta).value("[debug]"),
                Level::Info => style.set_color(Color::Cyan).value("[info]"),
                Level::Warn => style.set_color(Color::Yellow).value("[warning]"),
                Level::Error => style.set_color(Color::Red).value("[error]"),
            },
            record.args(),
        )
    }).init();
}
