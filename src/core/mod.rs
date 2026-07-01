//! Shared application primitives used across domains.

mod config;
mod error;
mod validation;

pub use config::AppConfig;
pub use error::CliError;
pub use validation::{ensure_clip_ids, ensure_destructive_confirmed};
