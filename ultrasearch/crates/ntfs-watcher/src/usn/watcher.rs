use anyhow::Result;
use core_types::{DocKey, VolumeId};
use std::path::Path;

#[derive(Debug)]
pub enum FileEvent {
    Change(DocKey),
    Delete(DocKey),
    Rename { old: DocKey, new: DocKey },
}

pub struct UsnWatcher {
    #[cfg(windows)]
    journal: usn_journal_rs::UsnJournal,
    volume_id: VolumeId,
}

impl UsnWatcher {
    pub fn new(volume_path: &Path, volume_id: VolumeId) -> Result<Self> {
        #[cfg(windows)]
        {
            let journal = usn_journal_rs::UsnJournal::open(volume_path)?;
            Ok(Self { journal, volume_id })
        }
        #[cfg(not(windows))]
        {
            Ok(Self { volume_id })
        }
    }

    /// Watch for changes starting from the given USN.
    /// This is a blocking iterator (or similar).
    /// For async, we'd need to wrap it or use a thread.
    pub fn watch(&mut self, _start_usn: u64) -> impl Iterator<Item = Result<FileEvent>> + '_ {
        std::iter::empty() // Stub implementation
    }
}
