use error_stack::{Report, Result, ResultExt};
use track::{
    error::{AppError, Suggestion},
    init,
};

fn main() -> Result<(), AppError> {
    init::error_reporting();

    Ok(())
}
