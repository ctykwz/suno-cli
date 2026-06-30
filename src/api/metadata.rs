use super::SunoClient;
use super::types::{
    ClipReaction, SetClipReactionRequest, SetMetadataRequest, SetVisibilityRequest,
};
use crate::core::CliError;

impl SunoClient {
    /// Update clip metadata (title, lyrics, caption, cover image).
    pub async fn set_metadata(
        &self,
        clip_id: &str,
        req: &SetMetadataRequest,
    ) -> Result<(), CliError> {
        self.with_auth_retry(|| async {
            let resp = self
                .post(&format!("/api/gen/{clip_id}/set_metadata/"))
                .json(req)
                .send()
                .await?;
            self.check_response(resp).await?;
            Ok(())
        })
        .await
    }

    /// Set clip visibility (public/private).
    pub async fn set_visibility(&self, clip_id: &str, is_public: bool) -> Result<(), CliError> {
        self.with_auth_retry(|| async {
            let resp = self
                .post(&format!("/api/gen/{clip_id}/set_visibility/"))
                .json(&SetVisibilityRequest { is_public })
                .send()
                .await?;
            self.check_response(resp).await?;
            Ok(())
        })
        .await
    }

    /// Set or clear a clip like/dislike reaction.
    pub async fn set_clip_reaction(
        &self,
        clip_id: &str,
        reaction: Option<ClipReaction>,
    ) -> Result<(), CliError> {
        self.with_auth_retry(|| async {
            let resp = self
                .post(&format!("/api/gen/{clip_id}/update_reaction_type/"))
                .json(&SetClipReactionRequest::new(reaction))
                .send()
                .await?;
            self.check_response(resp).await?;
            Ok(())
        })
        .await
    }

    /// Get word-level timestamped lyrics for a clip.
    pub async fn aligned_lyrics(
        &self,
        clip_id: &str,
    ) -> Result<Vec<super::types::AlignedWord>, CliError> {
        let body: serde_json::Value = self
            .with_auth_retry(|| async {
                let resp = self
                    .get(&format!("/api/gen/{clip_id}/aligned_lyrics/v2/"))
                    .send()
                    .await?;
                let resp = self.check_response(resp).await?;
                Ok(resp.json().await?)
            })
            .await?;
        // API returns {"aligned_words": [...], ...} — extract the array
        let words = body.get("aligned_words").ok_or_else(|| CliError::Api {
            code: "missing_field",
            message: "aligned_lyrics response missing 'aligned_words' field".into(),
        })?;
        Ok(serde_json::from_value(words.clone())?)
    }
}
