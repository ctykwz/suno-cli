use std::collections::HashMap;

use super::types::BrowserAuth;
use crate::core::CliError;

fn strip_cookie_header_prefix(input: &str) -> &str {
    let trimmed = input.trim();
    if trimmed.len() >= "cookie:".len()
        && trimmed[.."cookie:".len()].eq_ignore_ascii_case("cookie:")
    {
        trimmed["cookie:".len()..].trim()
    } else {
        trimmed
    }
}

fn parse_cookie_header(input: &str) -> HashMap<String, String> {
    strip_cookie_header_prefix(input)
        .split(';')
        .filter_map(|part| {
            let (name, value) = part.trim().split_once('=')?;
            let name = name.trim();
            if name.is_empty() {
                return None;
            }
            Some((name.to_string(), value.trim().to_string()))
        })
        .collect()
}

pub(super) fn sanitize_device_id(value: &str) -> Option<String> {
    let sanitized = value
        .trim()
        .replace("%22", "\"")
        .trim_matches('"')
        .trim_matches('\'')
        .trim()
        .to_string();
    if sanitized.is_empty() || sanitized.contains(';') {
        None
    } else {
        Some(sanitized)
    }
}

pub fn normalize_cookie_input(input: &str) -> Result<BrowserAuth, CliError> {
    let normalized = strip_cookie_header_prefix(input);
    let cookies = parse_cookie_header(normalized);

    if let Some(clerk_client_cookie) = cookies.get("__client").filter(|v| !v.is_empty()) {
        let device_id = cookies
            .get("ajs_anonymous_id")
            .and_then(|v| sanitize_device_id(v));
        return Ok(BrowserAuth {
            clerk_client_cookie: clerk_client_cookie.clone(),
            cookie_header: normalized.to_string(),
            device_id,
        });
    }

    if normalized.contains(';') || normalized.contains('=') {
        return Err(CliError::Config(
            "cookie header did not contain a __client field".into(),
        ));
    }

    let clerk_client_cookie = normalized.trim().to_string();
    if clerk_client_cookie.is_empty() {
        return Err(CliError::Config("empty Clerk __client cookie".into()));
    }
    Ok(BrowserAuth {
        cookie_header: format!("__client={clerk_client_cookie}"),
        clerk_client_cookie,
        device_id: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_raw_client_cookie() {
        let auth = normalize_cookie_input("client_token").unwrap();
        assert_eq!(auth.clerk_client_cookie, "client_token");
        assert_eq!(auth.cookie_header, "__client=client_token");
        assert!(auth.device_id.is_none());
    }

    #[test]
    fn normalizes_full_cookie_header_and_device() {
        let auth = normalize_cookie_input(
            "Cookie: foo=bar; __client=client_token; ajs_anonymous_id=%22device-123%22",
        )
        .unwrap();
        assert_eq!(auth.clerk_client_cookie, "client_token");
        assert_eq!(auth.device_id.as_deref(), Some("device-123"));
        assert!(auth.cookie_header.contains("__client=client_token"));
    }

    #[test]
    fn rejects_cookie_header_without_client() {
        let err = normalize_cookie_input("foo=bar; ajs_anonymous_id=device").unwrap_err();
        assert!(err.to_string().contains("__client"));
    }
}
