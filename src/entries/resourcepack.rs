use crate::utils::PathUtils;
use crate::utils;
use super::*;

#[derive(Clone, Debug, Deref, From)]
pub struct ResourcepackEntry(PathBuf);

impl Packageable for ResourcepackEntry {}

impl<S: Storage> Visitor<ResourcepackEntry> for Packager<S> {
    fn visit(&self, entry: &ResourcepackEntry) -> Result<()> {
        let to = PathBuf::from("resources.zip").prefix(self.config.dirname.as_ref());

        match entry.is_file() {
            true => self.target.copy(entry, &to),
            false => self.target.write(&to, &utils::create_zip_from_dir(entry)?),
        }
    }
}
