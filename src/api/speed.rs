use serde::Serialize;

use super::SunoClient;
use super::types::Clip;
use crate::core::CliError;

#[derive(Serialize)]
struct SpeedAdjustRequest<'a> {
    clip_id: &'a str,
    speed_multiplier: f64,
    keep_pitch: bool,
    title: &'a str,
}

impl SunoClient {
    /// Adjust clip playback speed through the current web edit route.
    pub async fn adjust_speed(
        &self,
        clip_id: &str,
        speed_multiplier: f64,
        keep_pitch: bool,
        title: &str,
    ) -> Result<Clip, CliError> {
        let req = SpeedAdjustRequest {
            clip_id,
            speed_multiplier,
            keep_pitch,
            title,
        };
        self.with_auth_retry(|| async {
            let resp = self
                .post("/api/clips/adjust-speed/")
                .json(&req)
                .send()
                .await?;
            let resp = self.check_response(resp).await?;
            Ok(resp.json().await?)
        })
        .await
    }
}
