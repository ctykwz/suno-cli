use super::SunoClient;
use super::types::ClipTrashRequest;
use crate::core::CliError;

impl SunoClient {
    pub async fn delete_clips(&self, ids: &[String]) -> Result<(), CliError> {
        self.set_clip_trash(ids, true).await
    }

    pub async fn restore_clips(&self, ids: &[String]) -> Result<(), CliError> {
        self.set_clip_trash(ids, false).await
    }

    async fn set_clip_trash(&self, ids: &[String], trash: bool) -> Result<(), CliError> {
        self.with_auth_retry(|| async {
            let resp = self
                .post("/api/gen/trash")
                .json(&ClipTrashRequest {
                    trash,
                    clip_ids: ids.to_vec(),
                })
                .send()
                .await?;
            self.check_response(resp).await?;
            Ok(())
        })
        .await
    }
}
