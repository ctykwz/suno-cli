use std::process::Stdio;
use std::sync::OnceLock;
use std::time::Duration;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::Mutex;
use tokio::time::sleep;

use super::{CDP_PORT, cdp};
use crate::core::CliError;

static CHROME: OnceLock<Mutex<Option<Child>>> = OnceLock::new();

fn chrome_slot() -> &'static Mutex<Option<Child>> {
    CHROME.get_or_init(|| Mutex::new(None))
}

/// Either reuse a Chrome instance already listening on `CDP_PORT` or spawn
/// a new hidden one with the sunox profile dir. Idempotent.
pub(super) async fn ensure_running() -> Result<(), CliError> {
    if cdp::cdp_version().await.is_ok() {
        return Ok(());
    }

    let chrome_path = locate_chrome()?;
    let profile_dir = directories::ProjectDirs::from("com", "sunox", "sunox")
        .map(|d| d.data_dir().join("chrome-profile"))
        .ok_or_else(|| CliError::Config("could not resolve data dir for chrome profile".into()))?;
    std::fs::create_dir_all(&profile_dir)?;

    eprintln!("Launching Chrome for captcha solver (one-time per session)...");

    // Do not use --headless. hCaptcha's bot-detection trips on headless mode.
    let mut child = Command::new(&chrome_path)
        .arg(format!("--remote-debugging-port={CDP_PORT}"))
        .arg(format!("--user-data-dir={}", profile_dir.display()))
        .arg("--no-first-run")
        .arg("--no-default-browser-check")
        .arg("--disable-search-engine-choice-screen")
        .arg("--disable-features=TranslateUI")
        .arg("--window-position=-32000,-32000")
        .arg("--window-size=1280,900")
        .arg("--silent-launch")
        .arg("about:blank")
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| CliError::Config(format!("failed to spawn Chrome at {chrome_path:?}: {e}")))?;
    drain_stderr(&mut child);

    {
        let mut slot = chrome_slot().lock().await;
        *slot = Some(child);
    }

    for _ in 0..20 {
        sleep(Duration::from_millis(500)).await;
        if cdp::cdp_version().await.is_ok() {
            return Ok(());
        }
    }

    Err(CliError::Config(
        "Chrome was spawned but never opened the CDP port. Check that Chrome can start normally, or set SUNO_CHROME_PATH to a Chrome/Chromium binary.".into(),
    ))
}

fn locate_chrome() -> Result<String, CliError> {
    if let Ok(path) = std::env::var("SUNO_CHROME_PATH")
        && !path.trim().is_empty()
    {
        if std::path::Path::new(&path).exists() {
            return Ok(path);
        }
        return Err(CliError::Config(format!(
            "SUNO_CHROME_PATH points to a missing file: {path}"
        )));
    }

    let candidates: &[&str] = if cfg!(target_os = "macos") {
        &[
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            "/Applications/Chromium.app/Contents/MacOS/Chromium",
            "/Applications/Brave Browser.app/Contents/MacOS/Brave Browser",
        ]
    } else if cfg!(target_os = "linux") {
        &[
            "/usr/bin/google-chrome",
            "/usr/bin/google-chrome-stable",
            "/usr/bin/chromium",
            "/usr/bin/chromium-browser",
            "/snap/bin/chromium",
        ]
    } else {
        &[
            "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
            "C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe",
        ]
    };

    for candidate in candidates {
        if std::path::Path::new(candidate).exists() {
            return Ok(candidate.to_string());
        }
    }
    Err(CliError::Config(
        "Could not find a Chrome/Chromium binary. Install Google Chrome or set SUNO_CHROME_PATH."
            .into(),
    ))
}

fn drain_stderr(child: &mut Child) {
    if let Some(stderr) = child.stderr.take() {
        let mut reader = BufReader::new(stderr).lines();
        tokio::spawn(async move {
            while let Ok(Some(_)) = reader.next_line().await {
                // discard
            }
        });
    }
}
