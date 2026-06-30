use super::SunoClient;
use super::types::{Clip, GenerateRequest};
use crate::core::CliError;

impl SunoClient {
    /// Create a cover of an existing clip.
    /// Posts to `/api/generate/v2-web/` with `cover_clip_id` set. Capture a
    /// fresh web request if Suno starts requiring extra cover fields such as
    /// `cover_start_s` or `cover_end_s`.
    pub async fn cover(
        &self,
        clip_id: &str,
        model_key: &str,
        tags: Option<&str>,
    ) -> Result<Vec<Clip>, CliError> {
        let mut req = GenerateRequest::new(model_key, "cover");
        req.tags = tags.map(String::from);
        req.cover_clip_id = Some(clip_id.to_string());
        self.generate(&req).await
    }
}
