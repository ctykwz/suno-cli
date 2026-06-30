use reqwest::Client;

use super::SunoClient;
use crate::auth::{self, AuthState};
use crate::core::CliError;

pub(super) async fn refresh_state_if_needed(
    client: &Client,
    auth: &mut AuthState,
) -> Result<(), CliError> {
    if !auth.is_jwt_expired() {
        return Ok(());
    }

    if let (Some(cookie), Some(session_id)) = (&auth.clerk_client_cookie, &auth.session_id) {
        eprintln!("JWT expired, refreshing via Clerk...");
        match auth::clerk_refresh_jwt(client, cookie, session_id).await {
            Ok(jwt) => {
                auth.jwt = Some(jwt);
                auth.save()?;
                eprintln!("JWT refreshed successfully");
                Ok(())
            }
            Err(e) => {
                eprintln!("JWT refresh failed: {e}");
                Err(CliError::AuthExpired)
            }
        }
    } else if let Some(cookie) = &auth.clerk_client_cookie {
        eprintln!("JWT expired, recovering Clerk session...");
        match auth::clerk_token_exchange(client, cookie).await {
            Ok((session_id, jwt)) => {
                auth.session_id = Some(session_id);
                auth.jwt = Some(jwt);
                auth.save()?;
                eprintln!("JWT refreshed successfully");
                Ok(())
            }
            Err(e) => {
                eprintln!("JWT refresh failed: {e}");
                Err(CliError::AuthExpired)
            }
        }
    } else {
        Err(CliError::AuthExpired)
    }
}

impl SunoClient {
    /// Refresh the JWT via the stored Clerk session cookie. Used by the
    /// in-process retry path in `with_auth_retry` when Suno's server-side
    /// staleness check fires mid-request despite a still-valid `exp` claim.
    pub(crate) async fn refresh_jwt(&self) -> Result<(), CliError> {
        let (cookie, session_id) = {
            let auth = self.auth.lock().expect("auth mutex poisoned");
            (
                auth.clerk_client_cookie
                    .clone()
                    .ok_or(CliError::AuthExpired)?,
                auth.session_id.clone(),
            )
        };
        let (session_id, jwt) = if let Some(session_id) = session_id {
            (
                session_id.clone(),
                auth::clerk_refresh_jwt(&self.client, &cookie, &session_id).await?,
            )
        } else {
            auth::clerk_token_exchange(&self.client, &cookie).await?
        };

        {
            let mut auth = self.auth.lock().expect("auth mutex poisoned");
            auth.session_id = Some(session_id);
            auth.jwt = Some(jwt);
            auth.save()?;
        }
        Ok(())
    }

    /// Run an async API call once. If it fails with `AuthExpired`, refresh
    /// the JWT and try a single retry.
    pub(crate) async fn with_auth_retry<F, Fut, T>(&self, mut f: F) -> Result<T, CliError>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, CliError>>,
    {
        match f().await {
            Err(CliError::AuthExpired) => {
                self.refresh_jwt().await?;
                f().await
            }
            other => other,
        }
    }
}
