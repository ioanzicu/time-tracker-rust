use error_stack::{Report, Result, ResultExt};
use tracing::{info, instrument, warn};
use track::{
    error::{AppError, Suggestion},
    init,
};

// RUST_LOG=warn cargo run
// RUST_LOG=trace cargo run

// track is the binary name
//
// track start
// track stop
// track report

// cargo nextest run
fn main() -> Result<(), AppError> {
    init::error_reporting();
    init::tracing();

    Ok(())
}
