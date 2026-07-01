//! Application entry layer: parse CLI input, build context, and dispatch commands.

mod context;
mod dispatch;
mod mutation_lock;

pub use context::AppContext;

use crate::core::CliError;

pub async fn run() -> Result<(), CliError> {
    dispatch::run().await
}
