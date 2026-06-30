use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD as BASE64URL;
use serde::{Deserialize, Serialize};

use crate::core::CliError;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AuthState {
    pub jwt: Option<String>,
    pub cookie: Option<String>,
    pub session_id: Option<String>,
    pub device_id: Option<String>,
    /// The __client cookie from clerk domain - long-lived (~7 days)
    pub clerk_client_cookie: Option<String>,
}

impl AuthState {
    pub fn load() -> Result<Self, CliError> {
        let path = Self::path();
        if !path.exists() {
            return Err(CliError::AuthMissing);
        }
        let data = std::fs::read_to_string(&path)?;
        serde_json::from_str(&data).map_err(|e| CliError::Config(format!("corrupt auth file: {e}")))
    }

    pub fn save(&self) -> Result<(), CliError> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let data = serde_json::to_string_pretty(self)?;
        let tmp = path.with_extension("json.tmp");

        #[cfg(unix)]
        {
            use std::io::Write;
            use std::os::unix::fs::OpenOptionsExt;
            let mut file = std::fs::OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .mode(0o600)
                .open(&tmp)?;
            file.write_all(data.as_bytes())?;
            file.sync_all()?;
        }

        #[cfg(not(unix))]
        {
            std::fs::write(&tmp, &data)?;
        }

        std::fs::rename(&tmp, &path)?;
        Ok(())
    }

    pub fn delete() -> Result<(), CliError> {
        let path = Self::path();
        if path.exists() {
            std::fs::remove_file(path)?;
        }
        Ok(())
    }

    pub fn is_jwt_expired(&self) -> bool {
        let Some(jwt) = &self.jwt else { return true };
        let parts: Vec<&str> = jwt.split('.').collect();
        if parts.len() != 3 {
            return true;
        }
        let claims = parts[1];
        let Ok(decoded) = BASE64URL.decode(claims) else {
            return true;
        };
        let Ok(value) = serde_json::from_slice::<serde_json::Value>(&decoded) else {
            return true;
        };
        let Some(exp) = value.get("exp").and_then(|v| v.as_u64()) else {
            return true;
        };
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        // Suno issues 1-hour JWTs, but generation can reject older tokens
        // before `exp`; refresh any JWT with under 30 minutes left.
        now + 1800 >= exp
    }

    fn path() -> PathBuf {
        directories::ProjectDirs::from("com", "sunox", "sunox")
            .map(|dirs| dirs.config_dir().join("auth.json"))
            .unwrap_or_else(|| PathBuf::from("~/.config/sunox/auth.json"))
    }
}
