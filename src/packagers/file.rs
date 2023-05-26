use std::path::Path;
use std::sync::Mutex;

use anyhow::Result;

use super::{FilePackager, Packager};
use crate::config::Config;
use crate::writers::Writer;

impl Packager for FilePackager {
    fn supports(&self, entry: &Path) -> bool {
        entry.is_file()
    }

    fn package(
        &self,
        entry: &Path,
        _: &Config,
        writer: &Mutex<Box<dyn Writer>>,
    ) -> Result<()> {
        writer.lock().unwrap().copy(entry)
    }
}
