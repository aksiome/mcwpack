use crate::utils::PrefixPath;
use super::*;

#[derive(Clone, Debug, Deref, From)]
pub struct FileEntry(PathBuf);

impl Packageable for FileEntry {}

impl<S: Storage> Visitor<FileEntry> for Packager<S> {
    fn visit(&self, entry: &FileEntry) -> Result<()> {
        self.target.copy(entry, &entry.prefix(self.config.dirname.as_ref()))
    }
}
