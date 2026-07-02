use std::time::Duration;

use reqwest::Client;

use crate::core::CliError;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
#[cfg(target_os = "macos")]
pub(crate) const BROWSER_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/149.0.0.0 Safari/537.36";
#[cfg(target_os = "windows")]
pub(crate) const BROWSER_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/149.0.0.0 Safari/537.36";
#[cfg(target_os = "linux")]
pub(crate) const BROWSER_USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/149.0.0.0 Safari/537.36";
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
pub(crate) const BROWSER_USER_AGENT: &str =
    "Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko) Chrome/149.0.0.0 Safari/537.36";
pub(crate) const BROWSER_ACCEPT_LANGUAGE: &str = "en";

pub fn browser_client() -> Result<Client, CliError> {
    Client::builder()
        .timeout(REQUEST_TIMEOUT)
        .user_agent(BROWSER_USER_AGENT)
        .build()
        .map_err(|e| CliError::Config(format!("HTTP client: {e}")))
}

pub fn default_client() -> Result<Client, CliError> {
    Client::builder()
        .timeout(REQUEST_TIMEOUT)
        .build()
        .map_err(|e| CliError::Config(format!("HTTP client: {e}")))
}
