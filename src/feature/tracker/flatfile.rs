//! A filesystem tracker

// flat file tracker
// 2 files:
// - "lockfile": tracker is running
// - "database file": JSON doc

use error_stack::{Result, ResultExt};
use std::{fs::OpenOptions, path::PathBuf};

#[derive(Debug, thiserror::Error)]
#[error("filesystem tracker error")]
pub struct FlatFileTrackerError;

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

    fn start(&self) -> Result<(), FlatFileTrackerError> {
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.lockfile)
            .change_context(FlatFileTrackerError)
            .attach_printable("unable to create new lockfile when starting trakcer")?;

        Ok(())
    }

    fn is_running(&self) -> bool {
        self.lockfile.exists()
    }

    fn stop(&self) -> Result<(), FlatFileTrackerError> {
        std::fs::remove_file(&self.lockfile)
            .change_context(FlatFileTrackerError)
            .attach_printable("unable to delete lockfile")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::process::Child;

    use assert_fs::{TempDir, fixture::ChildPath, prelude::PathChild};

    use super::*;

    fn tracking_paths() -> (TempDir, ChildPath, ChildPath) {
        let temp: TempDir = TempDir::new().unwrap();
        let db: ChildPath = temp.child("db.json");
        let lockfile: ChildPath = temp.child("lockfile");
        (temp, db, lockfile)
    }

    fn new_flat_file_tracker(db: &ChildPath, lockfile: &ChildPath) -> FlatFileTracker {
        FlatFileTracker::new(db.to_path_buf(), lockfile.to_path_buf())
    }

    #[test]
    fn is_running_returns_true_after_starting_tracker() {
        let (_tempdir, db, lockfile) = tracking_paths();

        // Given a default tracker
        let tracker: FlatFileTracker =
            FlatFileTracker::new(db.to_path_buf(), lockfile.to_path_buf());

        // When the tracker is started
        tracker.start().unwrap();

        // Then the tracker is running
        assert!(tracker.is_running());
    }

    #[test]
    fn is_running_returns_false_after_stopping_tracker() {
        // Given a new tracker that is running
        let (_tempdir, db, lockfile) = tracking_paths();
        let tracker: FlatFileTracker = new_flat_file_tracker(&db, &lockfile);
        tracker.start().unwrap();

        // When the tracker is started
        tracker.stop();

        // Then the tracker is no longer running
        assert!(!tracker.is_running());
    }
}
