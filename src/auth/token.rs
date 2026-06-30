use std::time::{SystemTime, UNIX_EPOCH};

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;

/// Generate the dynamic browser-token header value.
pub fn browser_token() -> String {
    let ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let payload = format!(r#"{{"timestamp":{ms}}}"#);
    let encoded = BASE64.encode(payload.as_bytes());
    format!(r#"{{"token":"{encoded}"}}"#)
}
