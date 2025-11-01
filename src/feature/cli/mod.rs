use error_stack::Result;
#[derive(Debug, thiserror::Error)]
#[error("a CLI error occured")]
pub struct CliError;

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Start,
    Stop,
    Report,
}

// no command -> err
// command -> no error

pub fn run() -> Result<(), CliError> {
    //
    todo!()
}
