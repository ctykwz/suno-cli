use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PromptUpsampleRequest<'a> {
    pub original_tags: &'a str,
    pub is_instrumental: bool,
}

#[derive(Debug, Deserialize)]
pub struct PromptUpsampleResponse {
    pub upsampled: String,
    pub request_id: String,
}
