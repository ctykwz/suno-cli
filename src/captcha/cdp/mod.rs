//! Chrome DevTools Protocol helpers for captcha solving.

mod session;
mod solver;
mod target;

pub(super) use solver::render_and_execute;
pub(super) use target::{cdp_version, find_or_create_suno_tab};
