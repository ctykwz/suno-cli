use std::fs::{File, OpenOptions};
use std::path::PathBuf;

use fs2::FileExt;

use super::AuthState;
use crate::core::CliError;

pub(crate) struct AuthRefreshLockGuard {
    file: File,
}

impl AuthRefreshLockGuard {
    pub(crate) fn acquire(auth: &AuthState) -> Result<Self, CliError> {
        let key = auth.account_lock_key()?;
        let path = lock_file_path(&key)?;
        let file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(path)?;
        file.lock_exclusive()?;
        Ok(Self { file })
    }
}

impl Drop for AuthRefreshLockGuard {
    fn drop(&mut self) {
        let _ = self.file.unlock();
    }
}

fn lock_file_path(key: &str) -> Result<PathBuf, CliError> {
    let dir = directories::ProjectDirs::from("com", "sunox", "sunox")
        .map(|dirs| dirs.config_dir().join("locks"))
        .ok_or_else(|| CliError::Config("cannot resolve sunox config directory".into()))?;
    std::fs::create_dir_all(&dir)?;
    Ok(dir.join(format!("auth-refresh-{key}.lock")))
}
