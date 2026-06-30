use std::collections::BTreeMap;

use reqwest::multipart::{Form, Part};

use super::SunoClient;
use super::types::{
    AudioUploadInitResponse, AudioUploadStatus, CreateAudioUploadRequest, FinishAudioUploadRequest,
    InitializeAudioClipRequest, InitializeAudioClipResponse,
};
use crate::core::CliError;

impl SunoClient {
    /// Start Suno's presigned audio upload flow.
    pub async fn create_audio_upload(
        &self,
        req: &CreateAudioUploadRequest,
    ) -> Result<AudioUploadInitResponse, CliError> {
        self.with_auth_retry(|| async {
            let resp = self.post("/api/uploads/audio/").json(req).send().await?;
            let resp = self.check_response(resp).await?;
            Ok(resp.json().await?)
        })
        .await
    }

    /// Upload local bytes to the presigned S3 form returned by Suno.
    pub async fn upload_presigned_audio_form(
        &self,
        url: &str,
        fields: &BTreeMap<String, String>,
        filename: &str,
        bytes: Vec<u8>,
    ) -> Result<(), CliError> {
        let mut form = Form::new();
        for (key, value) in fields {
            form = form.text(key.clone(), value.clone());
        }
        form = form.part("file", Part::bytes(bytes).file_name(filename.to_string()));

        let resp = self.client.post(url).multipart(form).send().await?;
        self.check_response(resp).await?;
        Ok(())
    }

    /// Mark a presigned audio upload as finished after the S3 form upload.
    pub async fn finish_audio_upload(
        &self,
        upload_id: &str,
        req: &FinishAudioUploadRequest,
    ) -> Result<(), CliError> {
        self.with_auth_retry(|| async {
            let resp = self
                .post(&format!("/api/uploads/audio/{upload_id}/upload-finish/"))
                .json(req)
                .send()
                .await?;
            self.check_response(resp).await?;
            Ok(())
        })
        .await
    }

    /// Fetch Suno's processing status for an uploaded audio file.
    pub async fn get_audio_upload(&self, upload_id: &str) -> Result<AudioUploadStatus, CliError> {
        self.with_auth_retry(|| async {
            let resp = self
                .get(&format!("/api/uploads/audio/{upload_id}/"))
                .send()
                .await?;
            let resp = self.check_response(resp).await?;
            Ok(resp.json().await?)
        })
        .await
    }

    /// Initialize a library clip from a completed audio upload.
    pub async fn initialize_audio_clip(
        &self,
        upload_id: &str,
        req: &InitializeAudioClipRequest,
    ) -> Result<InitializeAudioClipResponse, CliError> {
        self.with_auth_retry(|| async {
            let resp = self
                .post(&format!("/api/uploads/audio/{upload_id}/initialize-clip/"))
                .json(req)
                .send()
                .await?;
            let resp = self.check_response(resp).await?;
            Ok(resp.json().await?)
        })
        .await
    }
}
