use crate::api::SunoClient;
use crate::auth::AuthState;
use crate::core::AppConfig;
use crate::core::CliError;
use crate::output::OutputFormat;

pub struct AppContext {
    pub fmt: OutputFormat,
    pub quiet: bool,
    pub config: AppConfig,
}

impl AppContext {
    pub fn new(json: bool, quiet: bool, config_overrides: &[String]) -> Result<Self, CliError> {
        Ok(Self {
            fmt: OutputFormat::detect(json),
            quiet,
            config: AppConfig::load_with_overrides(config_overrides)?,
        })
    }

    pub async fn client(&self) -> Result<SunoClient, CliError> {
        let auth = AuthState::load()?;
        SunoClient::new_with_refresh(auth).await
    }
}
