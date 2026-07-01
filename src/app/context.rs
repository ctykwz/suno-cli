use crate::api::SunoClient;
use crate::auth::AuthState;
use crate::core::AppConfig;
use crate::core::CliError;
use crate::output::OutputFormat;

use super::mutation_lock::MutationLockGuard;

pub struct AppContext {
    pub fmt: OutputFormat,
    pub quiet: bool,
    pub parallel: bool,
    pub config: AppConfig,
}

impl AppContext {
    pub fn new(
        json: bool,
        quiet: bool,
        parallel: bool,
        config_overrides: &[String],
    ) -> Result<Self, CliError> {
        Ok(Self {
            fmt: OutputFormat::detect(json),
            quiet,
            parallel,
            config: AppConfig::load_with_overrides(config_overrides)?,
        })
    }

    pub async fn client(&self) -> Result<SunoClient, CliError> {
        let auth = AuthState::load()?;
        SunoClient::new_with_refresh(auth).await
    }

    pub fn should_lock_mutations(&self) -> bool {
        !self.parallel && self.config.serial_mutations
    }

    pub fn acquire_mutation_lock(&self) -> Result<Option<MutationLockGuard>, CliError> {
        if !self.should_lock_mutations() {
            return Ok(None);
        }

        let auth = AuthState::load()?;
        MutationLockGuard::acquire(&auth).map(Some)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::AppConfig;
    use crate::output::OutputFormat;

    use super::AppContext;

    fn context(parallel: bool, serial_mutations: bool) -> AppContext {
        let config = AppConfig {
            serial_mutations,
            ..Default::default()
        };
        AppContext {
            fmt: OutputFormat::Json,
            quiet: true,
            parallel,
            config,
        }
    }

    #[test]
    fn mutation_lock_policy_respects_parallel_override() {
        assert!(!context(true, true).should_lock_mutations());
    }

    #[test]
    fn mutation_lock_policy_respects_serial_mutations_config() {
        assert!(!context(false, false).should_lock_mutations());
    }

    #[test]
    fn mutation_lock_policy_defaults_to_locking() {
        assert!(context(false, true).should_lock_mutations());
    }
}
