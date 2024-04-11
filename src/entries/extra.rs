use std::ops::Deref;

use serde::Deserialize;

use super::*;

#[derive(Clone, Debug, Deserialize, From)]
#[serde(untagged)]
pub enum ExtraEntry {
    Short(PathBuf),
    Full(PathBuf, PathBuf),
}

impl Deref for ExtraEntry {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        match self {
            ExtraEntry::Short(path) => path,
            ExtraEntry::Full(path, _) => path,
        }
    }
}

impl ExtraEntry {
    pub fn canonicalize(&self) -> Result<ExtraEntry, std::io::Error> {
        Ok(match self {
            ExtraEntry::Short(path) => ExtraEntry::Short(path.canonicalize()?),
            ExtraEntry::Full(from, to) => ExtraEntry::Full(
                from.canonicalize()?,
                to.to_owned(),
            ),
        })
    }
}

impl Packageable for ExtraEntry {}

impl<S: Storage> Visitor<ExtraEntry> for Packager<S> {
    fn visit(&self, entry: &ExtraEntry) -> Result<()> {
        let to = match entry {
            ExtraEntry::Short(path) => PathBuf::from(path.file_name().unwrap_or_default()),
            ExtraEntry::Full(_, to) => to.to_owned(),
        };

        match entry.is_file() {
            true => self.target.copy(entry, &to),
            false => self.target.copy_dir_recursive(entry, &to),
        }
    }
}
