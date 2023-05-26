use std::path::Path;

use anyhow::Result;

pub mod dir;
pub mod zip;

pub trait Writer: Send + Sync {
    /// Copy an entry (file or directory) into the writer.
    fn copy(&mut self, entry: &Path) -> Result<()>;

    /// Write the contents buffer into the writer.
    fn write(&mut self, name: &Path, contents: Vec<u8>) -> Result<()>;
}
