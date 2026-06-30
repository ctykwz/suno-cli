use std::time::Duration;

use serde::Deserialize;
use tokio::time::sleep;

use crate::captcha::{CDP_HOST, CDP_PORT};
use crate::core::CliError;

#[derive(Debug, Deserialize)]
pub(in crate::captcha) struct Target {
    #[serde(rename = "type")]
    target_type: String,
    url: String,
    #[serde(rename = "webSocketDebuggerUrl")]
    pub(in crate::captcha) web_socket_debugger_url: String,
}

pub(in crate::captcha) async fn cdp_version() -> Result<serde_json::Value, CliError> {
    let url = format!("http://{CDP_HOST}:{CDP_PORT}/json/version");
    let resp = reqwest::Client::new()
        .get(&url)
        .timeout(Duration::from_secs(2))
        .send()
        .await
        .map_err(|e| CliError::Config(format!("CDP /json/version: {e}")))?;
    let version: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| CliError::Config(format!("CDP json parse: {e}")))?;
    Ok(version)
}

async fn cdp_list() -> Result<Vec<Target>, CliError> {
    let url = format!("http://{CDP_HOST}:{CDP_PORT}/json/list");
    let resp = reqwest::Client::new()
        .get(&url)
        .timeout(Duration::from_secs(5))
        .send()
        .await
        .map_err(|e| CliError::Config(format!("CDP /json/list: {e}")))?;
    let list: Vec<Target> = resp
        .json()
        .await
        .map_err(|e| CliError::Config(format!("CDP json parse: {e}")))?;
    Ok(list)
}

pub(in crate::captcha) async fn find_or_create_suno_tab() -> Result<Target, CliError> {
    let targets = cdp_list().await?;
    if let Some(target) = targets.into_iter().find(|target| {
        target.target_type == "page"
            && !target.web_socket_debugger_url.is_empty()
            && !target.url.starts_with("chrome://")
    }) {
        return Ok(target);
    }

    let url = format!(
        "http://{CDP_HOST}:{CDP_PORT}/json/new?{}",
        urlencode("about:blank")
    );
    let resp = reqwest::Client::new()
        .put(&url)
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .map_err(|e| CliError::Config(format!("CDP /json/new: {e}")))?;
    let target: Target = resp
        .json()
        .await
        .map_err(|e| CliError::Config(format!("CDP /json/new parse: {e}")))?;
    sleep(Duration::from_millis(800)).await;
    Ok(target)
}

fn urlencode(s: &str) -> String {
    s.replace(":", "%3A").replace("/", "%2F")
}
