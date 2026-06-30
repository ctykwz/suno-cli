use serde::{Deserialize, Serialize};

use super::clip::Clip;

/// Schema used by Suno's web generation endpoint `/api/generate/v2-web/`.
/// Placeholder fields must be present or Suno's server-side schema rejects
/// the request.
#[derive(Debug, Serialize)]
pub struct GenerateRequest {
    /// Optional anti-bot challenge token. Suno accepts many authenticated
    /// generation requests without one; callers can still force or supply a
    /// solved token when an account/session is challenged.
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<String>,
    pub generation_type: String,
    pub title: Option<String>,
    pub tags: Option<String>,
    /// Always present, defaults to an empty string.
    pub negative_tags: String,
    pub mv: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpt_description_prompt: Option<String>,
    pub make_instrumental: bool,
    pub user_uploaded_images_b64: Option<String>,
    pub metadata: GenerateMetadata,
    /// Always present, empty array unless overriding model fields.
    pub override_fields: Vec<String>,
    pub cover_clip_id: Option<String>,
    pub cover_start_s: Option<f64>,
    pub cover_end_s: Option<f64>,
    pub persona_id: Option<String>,
    pub artist_clip_id: Option<String>,
    pub artist_start_s: Option<f64>,
    pub artist_end_s: Option<f64>,
    pub continue_clip_id: Option<String>,
    pub continued_aligned_prompt: Option<String>,
    pub continue_at: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stem_type_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stem_type_group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stem_task: Option<String>,
    /// Random UUID generated per request.
    pub transaction_uuid: String,
    pub token_provider: Option<u8>,
}

impl GenerateRequest {
    pub fn new(mv: &str, create_mode: &str) -> Self {
        Self {
            token: None,
            task: None,
            generation_type: "TEXT".to_string(),
            title: None,
            tags: None,
            negative_tags: String::new(),
            mv: mv.to_string(),
            prompt: String::new(),
            gpt_description_prompt: None,
            make_instrumental: false,
            user_uploaded_images_b64: None,
            metadata: GenerateMetadata::new(create_mode),
            override_fields: Vec::new(),
            cover_clip_id: None,
            cover_start_s: None,
            cover_end_s: None,
            persona_id: None,
            artist_clip_id: None,
            artist_start_s: None,
            artist_end_s: None,
            continue_clip_id: None,
            continued_aligned_prompt: None,
            continue_at: None,
            stem_type_id: None,
            stem_type_group_name: None,
            stem_task: None,
            transaction_uuid: uuid::Uuid::new_v4().to_string(),
            token_provider: None,
        }
    }

    pub fn set_challenge_token(&mut self, token: Option<String>) {
        self.token = token;
        self.token_provider = self.token.as_ref().map(|_| 1);
    }
}

#[derive(Debug, Serialize)]
pub struct GenerateMetadata {
    pub web_client_pathname: String,
    pub is_max_mode: bool,
    pub is_mumble: bool,
    pub create_mode: String,
    pub user_tier: String,
    /// Random UUID generated per request.
    pub create_session_token: String,
    pub disable_volume_normalization: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_sliders: Option<ControlSliders>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lyrics_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_remix: Option<bool>,
}

impl GenerateMetadata {
    pub fn new(create_mode: &str) -> Self {
        Self {
            web_client_pathname: "/create".to_string(),
            is_max_mode: false,
            is_mumble: false,
            create_mode: create_mode.to_string(),
            user_tier: String::new(),
            create_session_token: uuid::Uuid::new_v4().to_string(),
            disable_volume_normalization: false,
            control_sliders: None,
            lyrics_model: None,
            is_remix: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ControlSliders {
    /// Weirdness: 0.0-1.0 (maps from 0-100 in UI)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weirdness_constraint: Option<f64>,
    /// Style weight: 0.0-1.0 (maps from 0-100 in this CLI)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_weight: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct GenerateResponse {
    #[serde(default)]
    pub clips: Vec<Clip>,
}
