use chrono::{DateTime, Utc};
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
