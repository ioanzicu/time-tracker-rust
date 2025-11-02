//! A filesystem tracker

// flat file tracker
// 2 files:
// - "lockfile": tracker is running
// - "database file": JSON doc

use error_stack::{Result, ResultExt};
use serde::{Deserialize, Serialize};
use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::{Path, PathBuf},
};

#[derive(Debug, thiserror::Error)]
#[error("filesystem tracker error")]
pub struct FlatFileTrackerError;

use crate::feature::tracker::{
    EndTime, Reporter, StartTime, StartupStatus, TimeRecord, Tracker, TrackerError,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LockfileData {
    start_time: StartTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct FlatFileDatabase {
    records: Vec<TimeRecord>,
}

impl FlatFileDatabase {
    pub fn push(&mut self, value: TimeRecord) {
        self.records.push(value)
    }
}

pub struct FlatFileTracker {
    db: PathBuf,
    lockfile: PathBuf,
}

impl Reporter for FlatFileTracker {}

impl FlatFileTracker {
    pub fn new<D, L>(db: D, lockfile: L) -> Self
    where
        D: Into<PathBuf>,
        L: Into<PathBuf>,
    {
        let db = db.into();
        let lockfile = lockfile.into();
        Self { db, lockfile }
    }

    fn start_impl(&self) -> Result<StartupStatus, FlatFileTrackerError> {
        if self.lockfile.exists() {
            Ok(StartupStatus::Running)
        } else {
            // Save the current start time into lockfile
            let lockfile_data = {
                let start_time = StartTime::now();
                let data = LockfileData { start_time };
                serde_json::to_string(&data)
                    .change_context(FlatFileTrackerError)
                    .attach_printable("failed to serialize lockfile data")?
            };

            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&self.lockfile)
                .change_context(FlatFileTrackerError)
                .attach_printable("unable to create new lockfile when starting trakcer")?
                .write_all(lockfile_data.as_bytes())
                .change_context(FlatFileTrackerError)
                .attach_printable("failed to write lockfile data")?;

            Ok(StartupStatus::Started)
        }
    }

    fn stop_impl(&self) -> Result<(), FlatFileTrackerError> {
        // 1. Read the time from the lockfile
        let start = read_lockfile(&self.lockfile)?.start_time;

        // 2. Get end time (EndTime::now())
        let end = EndTime::now();

        // 3. Create record
        let record = TimeRecord { start, end };

        // 4. Save the record
        let mut db = load_database(&self.db)?;
        db.push(record);
        save_database(&self.db, &db).unwrap();

        std::fs::remove_file(&self.lockfile)
            .change_context(FlatFileTrackerError)
            .attach_printable("unable to delete lockfile")?;

        Ok(())
    }
}

impl Tracker for FlatFileTracker {
    fn start(&mut self) -> Result<StartupStatus, TrackerError> {
        // Two states:
        // - startup from not running
        // - startup while already running
        self.start_impl().change_context(TrackerError)
    }

    fn is_running(&self) -> bool {
        self.lockfile.exists()
    }

    fn stop(&mut self) -> Result<(), TrackerError> {
        self.stop_impl().change_context(TrackerError)
    }

    fn records(&self) -> Result<impl Iterator<Item = TimeRecord>, TrackerError> {
        // 1. Load records
        let db = load_database(&self.db).change_context(TrackerError)?;
        // 2. Return iterator
        Ok(db.records.into_iter())
    }
}

fn save_database<P>(path: P, db: &FlatFileDatabase) -> Result<(), TrackerError>
where
    P: AsRef<Path>,
{
    let db = serde_json::to_string(&db)
        .change_context(TrackerError)
        .attach_printable("failed to serialize database")?;

    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(path.as_ref())
        .change_context(TrackerError)
        .attach_printable("failed to open database")?
        .write_all(db.as_bytes())
        .change_context(TrackerError)
        .attach_printable("failed to write database")?;

    Ok(())
}

fn load_database<P>(db: P) -> Result<FlatFileDatabase, FlatFileTrackerError>
where
    P: AsRef<Path>,
{
    let mut db_buf = String::default();

    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(db.as_ref())
        .change_context(FlatFileTrackerError)
        .attach_printable("failed to open database")?
        .read_to_string(&mut db_buf)
        .change_context(FlatFileTrackerError)
        .attach_printable("failed to read database")?;

    if db_buf.is_empty() {
        Ok(FlatFileDatabase::default())
    } else {
        Ok(serde_json::from_str(&db_buf)
            .change_context(FlatFileTrackerError)
            .attach_printable("failed to deserialize database")?)
    }
}

fn read_lockfile<P>(lockfile: P) -> Result<LockfileData, FlatFileTrackerError>
where
    P: AsRef<Path>,
{
    let file = OpenOptions::new()
        .read(true)
        .open(lockfile.as_ref())
        .change_context(FlatFileTrackerError)
        .attach_printable("failed to open lockfile")?;

    serde_json::from_reader(file)
        .change_context(FlatFileTrackerError)
        .attach_printable("failed to deserialize lockfile")
}

#[cfg(test)]
mod tests {
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
        let mut tracker: FlatFileTracker =
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
        let mut tracker: FlatFileTracker = new_flat_file_tracker(&db, &lockfile);
        tracker.start().unwrap();

        // When the tracker is started
        tracker.stop().unwrap();

        // Then the tracker is no longer running
        assert!(!tracker.is_running());
    }

    #[test]
    fn time_record_created_when_tracker_stops() {
        // Given a new tracker that is running
        let (_tempdir, db, lockfile) = tracking_paths();
        let mut tracker: FlatFileTracker = new_flat_file_tracker(&db, &lockfile);
        tracker.start().unwrap();

        // When the tracker is started
        tracker.stop().unwrap();

        // Then a record is saved
        // Iter<Record>
        assert!(tracker.records().unwrap().next().is_some());
    }

    #[test]
    fn multiple_starts_returns_already_running_state() {
        // Given a new tracker that is running
        let (_tempdir, db, lockfile) = tracking_paths();
        let mut tracker: FlatFileTracker = new_flat_file_tracker(&db, &lockfile);
        tracker.start().unwrap();

        // When the tracker is started again
        let started = tracker.start().unwrap();

        // Then the "alread running" state is returned
        assert_eq!(started, StartupStatus::Running);
    }

    #[test]
    fn initial_starts_returns_started_state() {
        // Given a new tracker that is running
        let (_tempdir, db, lockfile) = tracking_paths();
        let mut tracker: FlatFileTracker = new_flat_file_tracker(&db, &lockfile);
        let started = tracker.start().unwrap();

        // Then the "started" state is returned
        assert_eq!(started, StartupStatus::Started);
    }
}
