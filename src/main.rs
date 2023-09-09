use std::io::Write;
use std::path::PathBuf;

use clap::Parser;
use log::{Level, LevelFilter};
use mcwpack::utils::*;
use mcwpack::Config;
use mcwpack::Context;

const DEFAULT_CONFIG: &str = "mcwpack.yaml";

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
    #[arg(short, long, conflicts_with = "quiet")]
    verbose: bool,
    /// Silence warning
    #[arg(short, long, conflicts_with = "verbose")]
    quiet: bool,
    /// Force colorization
    #[arg(long)]
    colors: bool,
    /// Ignore prompts
    #[arg(long)]
    noprompt: bool,
}

fn main() {
    let opts = Opts::parse();

    if opts.colors {
        console::set_colors_enabled(true);
        console::set_colors_enabled_stderr(true);
    }

    let verbosity = match (opts.verbose, opts.quiet) {
        (true, _) => LevelFilter::Trace,
        (_, true) => LevelFilter::Error,
        _ => LevelFilter::Warn,
    };

    env_logger::builder().format(|buf, record| {
        writeln!(buf, "{} {}", match record.level() {
            Level::Trace => console::style("(-)").black().bold(),
            Level::Debug => console::style("(>)").magenta().bold(),
            Level::Info => console::style("(i)").cyan().bold(),
            Level::Warn => console::style("(!)").yellow().bold(),
            Level::Error => console::style("(x)").red().bold(),
        }, match record.level() {
            Level::Warn => console::style(record.args()).yellow(),
            Level::Error => console::style(record.args()).red(),
            _ => console::style(record.args()),
        })
    }).filter(Some("mcwpack"), verbosity).init();

    let world = opts.world.to_owned().map(|p| p.canonicalize().unwrap_or_else(|err| {
        log::error!("the world path is not valid ({})", err);
        std::process::exit(1);
    })).unwrap_or_else(|| {
        if opts.noprompt {
            log::error!("a world path must be provided when noprompt is enabled");
            std::process::exit(1);
        }
        enter_path("Please enter the world path: ", true).canonicalize().unwrap()
    });

    let config = opts.config.to_owned().unwrap_or_else(|| world.join(DEFAULT_CONFIG));

    if let Some(config) = Config::load(&config, opts.noprompt) {
        let (context, target) = match (opts.dir, opts.zip) {
            (Some(path), _) => (Context::Dir, path),
            (_, Some(path)) => (Context::Zip, path.with_extension("zip")),
            _ if opts.noprompt => {
                log::error!("a target path must be provided when noprompt is enabled (use either -z or -d)");
                std::process::exit(1);
            },
            _ => (Context::Zip, enter_path("Please enter the zip output path: ", false).with_extension("zip"))
        };

        if !target.exists() || !opts.noprompt && match context {
            Context::Dir => confirm("The output directory already exists, do you want to continue?", true),
            Context::Zip => confirm("The output zip file already exists, do you want to replace it?", true),
        } {
            context.package(config, &world, &target);
        };
    }
}
