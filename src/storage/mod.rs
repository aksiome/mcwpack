pub use self::dir::DirStorage;
pub use self::zip::ZipStorage;

mod dir;
mod zip;

use std::path::Path;

use anyhow::Result;
use ignore::WalkBuilder;

pub trait InMemoryStorage: Storage {
    fn new(buffer: &[u8]) -> Self;
}

pub trait FilesystemStorage: Storage {
    fn new(path: &Path) -> Self;
}

pub trait Storage: Send + Sync {
    /// Get the path to the storage.
    fn path(&self) -> Option<&Path>;

    /// Copy a file into the storage.
    fn copy(&self, from: &Path, to: &Path) -> Result<()>;

    /// Write a file into the storage.
    fn write(&self, file: &Path, contents: &[u8]) -> Result<()>;

    /// Recursively copy a directory into the storage.
    fn copy_dir_recursive(&self, from: &Path, to: &Path) -> Result<()> {
        let walker = WalkBuilder::new(from).same_file_system(true).build();
        for entry in walker.into_iter().filter_map(|e| e.ok()) {
            self.copy(entry.path(), &to.join(entry.path().strip_prefix(from)?))?;
        }

        Ok(())
    }
}
