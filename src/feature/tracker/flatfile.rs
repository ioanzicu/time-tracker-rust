//! A filesystem tracker

// flat file tracker
// 2 files:
// - "lockfile": tracker is running
// - "database file": JSON doc

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
#[error("filesystem tracker error")]
pub struct FlatFileTracker {
    db: PathBuf,
    lockfile: PathBuf,
}

impl FlatFileTracker {
    fn new<D, L>(db: D, lockfile: L) -> Self
    where
        D: Into<PathBuf>,
        L: Into<PathBuf>,
    {
        let db = db.into();
        let lockfile = lockfile.into();
        Self { db, lockfile }
    }

    fn start(&self) -> bool {
        todo!()
    }

    fn is_running(&self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_tracking_with_default_tracker() {
        // Given a default tracker
        let tracker = FlatFileTracker::new("db.json", "lockfile");

        // When the tracker is started
        tracker.start();
        // Then the tracker is running
        assert!(tracker.is_running());
    }
}
