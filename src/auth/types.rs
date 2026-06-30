#[derive(Debug, Clone)]
pub struct BrowserAuth {
    pub clerk_client_cookie: String,
    pub cookie_header: String,
    pub device_id: Option<String>,
}
