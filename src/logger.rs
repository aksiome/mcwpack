use env_logger::filter::{Builder, Filter};
use log::{LevelFilter, Log, Metadata, Record, Level};

use crate::app::PROGRESS;

pub struct Logger {
    inner: Filter,
}

impl Logger {
    pub fn new(level: LevelFilter) -> Self {
        Self {
            inner: Builder::new().filter(Some("mcwpack"), level).build(),
        }
    }

    pub fn init(level: LevelFilter) {
        let logger = Self::new(level);
        log::set_max_level(level);
        log::set_boxed_logger(Box::new(logger)).expect("Logger::init should not be called more than once")
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.inner.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        if self.inner.matches(record) {
            let msg = format!("{}{} {}", match PROGRESS.is_hidden() {
                true => "",
                false => "   ",
            }, match record.level() {
                Level::Trace => console::style("(-)").black().bold(),
                Level::Debug => console::style("(>)").magenta().bold(),
                Level::Info => console::style("(i)").cyan().bold(),
                Level::Warn => console::style("(!)").yellow().bold(),
                Level::Error => console::style("(x)").red().bold(),
            }, match record.level() {
                Level::Warn => console::style(record.args()).yellow(),
                Level::Error => console::style(record.args()).red(),
                _ => console::style(record.args()),
            });

            match PROGRESS.is_hidden() {
                true => println!("{}", msg),
                false => PROGRESS.println(msg),
            };
        }
    }

    fn flush(&self) {}
}
