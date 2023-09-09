use crate::formats::Region;
use crate::utils::PrefixPath;
use crate::PB_SPACING;
use super::*;

#[derive(Clone, Debug, Deref, From)]
pub struct RegionEntry(PathBuf);

impl Packageable for RegionEntry {}

impl<S: Storage> Visitor<RegionEntry> for Packager<S> {
    fn visit(&self, entry: &RegionEntry) -> Result<()> {
        if entry.metadata()?.len() <= 8192 {
            self.progress.suspend(|| {
                log::info!("{}skipped empty region [{}]", PB_SPACING, entry.display())
            });
            return Ok(());
        }

        let to = entry.prefix(self.config.dirname.as_ref());

        if self.config.clean_chunks {
            let contents = Region::load(entry)
                .with_context(|| "could not read region")?
                .optimize_bytes(&self.config)
                .with_context(|| "could not process region")?;

            if contents.len() <= 8192 {
                self.progress.suspend(|| {
                    log::info!("{}skipped empty region [{}]", PB_SPACING, entry.display())
                });
                return Ok(());
            }

            return self.target.write(&to, &contents);
        }

        self.target.copy(entry, &to)
    }
}
