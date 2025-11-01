use std::path::PathBuf;

use clap::{Parser, Subcommand};
use error_stack::{Result, ResultExt};

use crate::{
    error::Suggestion,
    feature::tracker::{FlatFileTracker, StartupStatus, Tracker},
};

#[derive(Debug, thiserror::Error)]
#[error("a CLI error occured")]
pub struct CliError;

#[derive(Debug, Clone, Copy, Subcommand)]
pub enum Command {
    /// Start tracking time
    Start,
    Stop,
    // Report,
}

#[derive(Parser, Debug)]
#[command(version, about, arg_required_else_help(true))]
pub struct Cli {
    /// Path to database file
    #[arg(short = 'd', long)]
    pub db_dir: Option<PathBuf>,

    /// Path to lockfile
    #[arg(short = 'l', long)]
    pub lockfile: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Command,
}

// no command -> err
// command -> no error

pub fn run() -> Result<(), CliError> {
    let args: Cli = Cli::parse();

    let db_dir = flatfile_db_dir(&args)?;
    let lockfile = lockfile_path(&args)?;

    let mut tracker = FlatFileTracker::new(db_dir, lockfile);

    match args.command {
        Command::Start => match tracker.start() {
            Ok(StartupStatus::Started) => (),
            Ok(StartupStatus::Running) => println!("tracker already running"),
            Err(e) => return Err(e).change_context(CliError),
        },
        Command::Stop => tracker
            .stop()
            .change_context(CliError)
            .attach_printable("failed to stop tracking")?,
    }

    Ok(())
}

fn flatfile_db_dir(args: &Cli) -> Result<PathBuf, CliError> {
    match &args.db_dir {
        Some(db_dir) => Ok(db_dir.clone()),
        None => {
            let mut db_dir = dirs::data_dir()
                .ok_or(CliError)
                .attach_printable("failed to discover directory")
                .attach(Suggestion("use the -d flag to specify a database path"))?;

            // ../home/some_dir/data/track
            db_dir.push("track");

            std::fs::create_dir_all(&db_dir)
                .change_context(CliError)
                .attach_printable("failed to create 'track' db directory")?;

            db_dir.push("records.json");
            Ok(db_dir)
        }
    }
}

fn lockfile_path(args: &Cli) -> Result<PathBuf, CliError> {
    match &args.lockfile {
        Some(lockfile) => Ok(lockfile.clone()),
        None => {
            let mut lockfile = dirs::cache_dir()
                .ok_or(CliError)
                .attach_printable("failed to discover cache directory")
                .attach(Suggestion("use the -l flag to specify a lockfile path"))?;

            // ../home/some_dir/track
            lockfile.push("track");

            std::fs::create_dir_all(&lockfile)
                .change_context(CliError)
                .attach_printable("failed to create 'track' cache directory")?;

            lockfile.push("track.lock");
            Ok(lockfile)
        }
    }
}
