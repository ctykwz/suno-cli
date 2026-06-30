//! Cookie sources and conversion for the browser-backed captcha solver.

use std::collections::HashSet;

use crate::auth::AuthState;
use crate::core::CliError;

mod browser;
mod cdp_cookie;

use browser::add_live_browser_cookies;
use cdp_cookie::{CdpCookie, add_minimal_cookies_from_header, push_cookie};

pub(super) fn extract_cookies(auth: &AuthState) -> Result<Vec<CdpCookie>, CliError> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();

    if add_live_browser_cookies(&mut out, &mut seen) && !out.is_empty() {
        return Ok(out);
    }

    if let Some(clerk) = auth
        .clerk_client_cookie
        .as_deref()
        .filter(|cookie| !cookie.trim().is_empty())
    {
        push_cookie(
            &mut out,
            &mut seen,
            "__client",
            clerk.trim(),
            "auth.suno.com",
            true,
        );
        push_cookie(
            &mut out,
            &mut seen,
            "__client",
            clerk.trim(),
            ".suno.com",
            true,
        );
    }

    if let Some(device_id) = auth
        .device_id
        .as_deref()
        .filter(|device_id| !device_id.trim().is_empty())
    {
        push_cookie(
            &mut out,
            &mut seen,
            "ajs_anonymous_id",
            device_id.trim(),
            ".suno.com",
            false,
        );
    }

    if let Some(cookie_header) = auth
        .cookie
        .as_deref()
        .filter(|cookie| !cookie.trim().is_empty())
    {
        add_minimal_cookies_from_header(cookie_header, &mut out, &mut seen);
    }

    if !out.is_empty() {
        return Ok(out);
    }

    if add_live_browser_cookies(&mut out, &mut seen) && !out.is_empty() {
        return Ok(out);
    }

    Ok(out)
}
