use crate::utils::PrefixPath;
use crate::utils;
use super::*;

#[derive(Clone, Debug, Deref, From)]
pub struct DatapackEntry(PathBuf);

impl Packageable for DatapackEntry {}

impl<S: Storage> Visitor<DatapackEntry> for Packager<S> {
    fn visit(&self, entry: &DatapackEntry) -> Result<()> {
        let to = entry.prefix(self.config.dirname.as_ref());

        if entry.is_file() {
            return self.target.copy(entry, &to);
        }

        match self.config.zip_datapacks {
            true => self.target.write(
                &to.with_extension("zip"),
                &utils::create_zip_from_dir(entry)?,
            ),
            false => self.target.copy_dir_recursive(entry, &to),
        }
    }
}
