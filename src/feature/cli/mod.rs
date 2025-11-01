use clap::{Parser, Subcommand};
use error_stack::Result;
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

    match args.command {
        Command::Start => todo!(),
    }

    Ok(())
}
