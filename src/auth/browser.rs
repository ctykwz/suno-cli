use std::collections::HashSet;

use super::cookie::sanitize_device_id;
use super::types::BrowserAuth;
use crate::core::CliError;

/// Extract Suno auth cookies from the user's browsers.
/// Tries Chrome, Arc, Brave, Firefox, and Edge in order.
pub fn extract_browser_auth() -> Result<BrowserAuth, CliError> {
    let domains = vec![
        "suno.com".into(),
        "auth.suno.com".into(),
        ".suno.com".into(),
    ];

    for (name, result) in [
        ("Chrome", rookie::chrome(Some(domains.clone()))),
        ("Arc", rookie::arc(Some(domains.clone()))),
        ("Brave", rookie::brave(Some(domains.clone()))),
        ("Firefox", rookie::firefox(Some(domains.clone()))),
        ("Edge", rookie::edge(Some(domains.clone()))),
    ] {
        if let Ok(cookies) = result {
            let mut seen = HashSet::new();
            let mut header_parts = Vec::new();
            let mut clerk_client_cookie: Option<String> = None;
            let mut auth_domain_clerk: Option<String> = None;
            let mut device_id: Option<String> = None;

            for cookie in cookies {
                if !cookie.domain.contains("suno.com") {
                    continue;
                }
                if cookie.name == "__client" && !cookie.value.is_empty() {
                    if cookie.domain.contains("auth.suno.com") {
                        auth_domain_clerk = Some(cookie.value.clone());
                    } else if clerk_client_cookie.is_none() {
                        clerk_client_cookie = Some(cookie.value.clone());
                    }
                }
                if cookie.name == "ajs_anonymous_id" && device_id.is_none() {
                    device_id = sanitize_device_id(&cookie.value);
                }
                let key = (cookie.name.clone(), cookie.domain.clone());
                if seen.insert(key) {
                    header_parts.push(format!("{}={}", cookie.name, cookie.value));
                }
            }

            if let Some(clerk_client_cookie) = auth_domain_clerk.or(clerk_client_cookie) {
                eprintln!("Found Suno session in {name}");
                return Ok(BrowserAuth {
                    clerk_client_cookie,
                    cookie_header: header_parts.join("; "),
                    device_id,
                });
            }
        }
    }

    Err(CliError::Config(
        "No Suno session found in any browser. Log into suno.com first, then retry.".into(),
    ))
}
