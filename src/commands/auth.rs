use crate::api::SunoClient;
use crate::app::AppContext;
use crate::auth::{self, AuthState};
use crate::cli::AuthArgs;
use crate::core::CliError;

pub async fn run(args: AuthArgs, _ctx: &AppContext) -> Result<(), CliError> {
    if args.logout {
        AuthState::delete()?;
        eprintln!("Logged out; removed stored Suno authentication");
        return Ok(());
    }

    let mut state = match AuthState::load() {
        Ok(s) => s,
        Err(CliError::AuthMissing) => AuthState::default(),
        Err(e) => return Err(e),
    };

    let has_explicit_auth_input =
        args.login || args.refresh || args.jwt.is_some() || args.cookie.is_some();
    let should_login = args.login
        || (!has_explicit_auth_input && state.jwt.is_none() && state.clerk_client_cookie.is_none());

    if args.refresh {
        let cookie = state.clerk_client_cookie.clone().ok_or_else(|| {
            CliError::Config("no Clerk session cookie stored — run `sunox login` first".into())
        })?;
        let http = reqwest::Client::new();
        eprintln!("Refreshing JWT via Clerk session cookie...");
        let (session_id, jwt) = if let Some(session_id) = state.session_id.clone() {
            (
                session_id.clone(),
                auth::clerk_refresh_jwt(&http, &cookie, &session_id).await?,
            )
        } else {
            auth::clerk_token_exchange(&http, &cookie).await?
        };
        state.session_id = Some(session_id);
        state.jwt = Some(jwt);
        state.save()?;
        eprintln!("JWT refreshed successfully");
    } else if should_login {
        eprintln!("Extracting Suno session from your browser...");
        let browser_auth = auth::extract_browser_auth()?;

        let http = reqwest::Client::new();
        eprintln!("Exchanging for access token via Clerk...");
        let (session_id, jwt) =
            auth::clerk_token_exchange(&http, &browser_auth.clerk_client_cookie).await?;

        state.cookie = Some(browser_auth.cookie_header);
        state.clerk_client_cookie = Some(browser_auth.clerk_client_cookie);
        state.session_id = Some(session_id);
        state.jwt = Some(jwt);
        state.device_id = browser_auth
            .device_id
            .or(state.device_id)
            .or_else(|| Some(uuid::Uuid::new_v4().to_string()));
    } else if let Some(cookie) = args.cookie.as_deref() {
        let browser_auth = auth::normalize_cookie_input(cookie)?;
        let http = reqwest::Client::new();
        eprintln!("Exchanging cookie for access token...");
        let (session_id, jwt) =
            auth::clerk_token_exchange(&http, &browser_auth.clerk_client_cookie).await?;

        state.cookie = Some(browser_auth.cookie_header);
        state.clerk_client_cookie = Some(browser_auth.clerk_client_cookie);
        state.session_id = Some(session_id);
        state.jwt = Some(jwt);
        state.device_id = browser_auth
            .device_id
            .or(state.device_id)
            .or_else(|| Some(uuid::Uuid::new_v4().to_string()));
    } else if let Some(jwt) = args.jwt.clone() {
        state.jwt = Some(jwt);
        if state.device_id.is_none() {
            state.device_id = Some(uuid::Uuid::new_v4().to_string());
        }
    } else {
        eprintln!("Checking existing authentication...");
    }

    if let Some(device) = args.device.as_ref() {
        state.device_id = Some(device.clone());
    }

    let should_save_after_verify = args.refresh
        || should_login
        || args.cookie.is_some()
        || args.jwt.is_some()
        || args.device.is_some();
    let client = SunoClient::new_with_refresh(state.clone()).await?;
    let info = client.billing_info().await?;
    if should_save_after_verify {
        state.save()?;
    }
    eprintln!(
        "Authenticated! Plan: {}, Credits: {}",
        info.plan.name, info.total_credits_left
    );
    Ok(())
}
