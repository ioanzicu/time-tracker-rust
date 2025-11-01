use clap::{Parser, Subcommand};
use error_stack::{Result, ResultExt};

use crate::feature::tracker::{FlatFileTracker, StartupStatus, Tracker};

#[derive(Debug, thiserror::Error)]
#[error("a CLI error occured")]
pub struct CliError;

#[derive(Debug, Clone, Copy, Subcommand)]
pub enum Command {
    /// Start tracking time
    Start,
    // Stop,
    // Report,
}

#[derive(Parser, Debug)]
#[command(version, about, arg_required_else_help(true))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

// no command -> err
// command -> no error

pub fn run() -> Result<(), CliError> {
    let args: Cli = Cli::parse();

    let mut tracker = FlatFileTracker::new("db.json", "lockfile");

    match args.command {
        Command::Start => match tracker.start() {
            Ok(StartupStatus::Started) => println!("tracking started"),
            Ok(StartupStatus::Running) => println!("tracker already running"),
            Err(e) => return Err(e).change_context(CliError),
        },
    }

    Ok(())
}
