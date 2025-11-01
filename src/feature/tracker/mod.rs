use chrono::{DateTime, Utc};
use error_stack::Result;
use serde::{Deserialize, Serialize};

mod flatfile;
// flat file tracker
// 2 files:
// - "lockfile": tracker is running
// - "database file": JSON doc

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StartTime(DateTime<Utc>);
impl StartTime {
    pub fn now() -> Self {
        Self(Utc::now())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EndTime(DateTime<Utc>);
impl EndTime {
    pub fn now() -> Self {
        Self(Utc::now())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TimeRecord {
    start: StartTime,
    end: EndTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StartupStatus {
    /// Time tracker started
    Started,
    /// Time tracker was already running
    Running,
}

#[derive(Debug, thiserror::Error)]
#[error("filesystem tracker error")]
pub struct TrackerError;

pub trait Tracker {
    fn start(&self) -> Result<StartupStatus, TrackerError>;

    fn is_running(&self) -> bool;

    fn stop(&self) -> Result<(), TrackerError>;

    fn records(&self) -> Result<impl Iterator<Item = TimeRecord>, TrackerError>;
}
