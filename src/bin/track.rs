use error_stack::{Result, ResultExt};
use track::{error::AppError, feature::cli, init};

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

    cli::run()
        .change_context(AppError)
        .attach_printable("failed to run CLI")?;

    Ok(())
}
