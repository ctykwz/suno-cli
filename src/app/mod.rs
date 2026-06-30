//! Application entry layer: parse CLI input, build context, and dispatch commands.

mod context;
mod dispatch;

pub use context::AppContext;

use crate::core::CliError;

pub async fn run() -> Result<(), CliError> {
    dispatch::run().await
}
