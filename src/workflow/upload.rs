use std::path::Path;
use std::time::{Duration, Instant};

use serde::Serialize;
use tokio::time::sleep;

use crate::api::SunoClient;
use crate::api::types::{
    Clip, CreateAudioUploadRequest, CreateAudioUploadSpec, FinishAudioUploadRequest,
    InitializeAudioClipRequest, SetMetadataRequest,
};
use crate::core::CliError;

#[derive(Debug, Serialize)]
pub struct UploadResult {
    pub upload_id: String,
    pub status_upload_id: Option<String>,
    pub clip_id: Option<String>,
    pub clip: Option<Clip>,
    pub has_vocal: Option<bool>,
    pub status: String,
}

pub struct UploadWorkflowInput<'a> {
    pub file: &'a Path,
    pub upload_type: &'a str,
    pub is_stem_mix: bool,
    pub title: Option<String>,
    pub lyrics: Option<String>,
    pub timeout: Duration,
    pub poll_interval: Duration,
}

pub async fn run(
    client: &SunoClient,
    input: UploadWorkflowInput<'_>,
) -> Result<UploadResult, CliError> {
    let extension = audio_extension(input.file)?;
    let filename = upload_filename(input.file)?;
    let bytes = tokio::fs::read(input.file).await?;

    let upload = client
        .create_audio_upload(&CreateAudioUploadRequest {
            spec: CreateAudioUploadSpec {
                extension,
                is_stem_mix: input.is_stem_mix,
                upload_type: input.upload_type.to_string(),
            },
        })
        .await?;

    client
        .upload_presigned_audio_form(&upload.url, &upload.fields, &filename, bytes)
        .await?;

    client
        .finish_audio_upload(
            &upload.id,
            &FinishAudioUploadRequest {
                upload_type: input.upload_type.to_string(),
                upload_filename: filename,
            },
        )
        .await?;

    let status =
        wait_until_complete(client, &upload.id, input.timeout, input.poll_interval).await?;
    let initialized = client
        .initialize_audio_clip(
            &upload.id,
            &InitializeAudioClipRequest {
                downbeats: None,
                user_reviewed_tags: Some(true),
            },
        )
        .await?;

    let clip_id = initialized_clip_id(&initialized)?;

    let title = input.title.or_else(|| status.title.clone());
    let lyrics = input.lyrics;
    if title.is_some() || lyrics.is_some() || status.image_url.is_some() {
        client
            .set_metadata(
                &clip_id,
                &SetMetadataRequest {
                    title,
                    lyrics,
                    caption: None,
                    image_url: status.image_url.clone(),
                    is_audio_upload_tos_accepted: Some(true),
                    remove_image_cover: None,
                    remove_video_cover: None,
                },
            )
            .await?;
    }

    Ok(UploadResult {
        upload_id: upload.id,
        status_upload_id: status.id,
        clip_id: Some(clip_id),
        clip: initialized.clip,
        has_vocal: status.has_vocal,
        status: "complete".into(),
    })
}

pub fn audio_extension(path: &Path) -> Result<String, CliError> {
    let extension = path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.trim_start_matches('.').to_ascii_lowercase())
        .filter(|extension| !extension.is_empty())
        .ok_or_else(|| CliError::Config("upload file must have an audio extension".into()))?;
    Ok(extension)
}

pub fn upload_filename(path: &Path) -> Result<String, CliError> {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(str::to_string)
        .filter(|name| !name.is_empty())
        .ok_or_else(|| CliError::Config("upload file must have a valid filename".into()))
}

fn initialized_clip_id(
    initialized: &crate::api::types::InitializeAudioClipResponse,
) -> Result<String, CliError> {
    initialized
        .clip_id
        .clone()
        .or_else(|| initialized.clip.as_ref().map(|clip| clip.id.clone()))
        .ok_or_else(|| CliError::Api {
            code: "schema_drift",
            message: "audio upload initialization completed without a clip id".into(),
        })
}

async fn wait_until_complete(
    client: &SunoClient,
    upload_id: &str,
    timeout: Duration,
    poll_interval: Duration,
) -> Result<crate::api::types::AudioUploadStatus, CliError> {
    let deadline = Instant::now() + timeout;
    let poll_interval = poll_interval.max(Duration::from_secs(1));
    loop {
        let status = client.get_audio_upload(upload_id).await?;
        match status.status.as_deref() {
            Some("complete") => return Ok(status),
            Some("error") => {
                return Err(CliError::GenerationFailed(format!(
                    "audio upload {upload_id} failed during processing"
                )));
            }
            _ if Instant::now() >= deadline => {
                return Err(CliError::GenerationFailed(format!(
                    "audio upload {upload_id} did not complete within {} seconds",
                    timeout.as_secs()
                )));
            }
            _ => sleep(poll_interval).await,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::api::types::InitializeAudioClipResponse;

    use super::{audio_extension, initialized_clip_id, upload_filename};

    #[test]
    fn audio_extension_lowercases_file_extension() {
        assert_eq!(
            audio_extension(Path::new("/tmp/Demo.MP3")).expect("extension"),
            "mp3"
        );
    }

    #[test]
    fn upload_filename_uses_basename() {
        assert_eq!(
            upload_filename(Path::new("/tmp/Demo.MP3")).expect("filename"),
            "Demo.MP3"
        );
    }

    #[test]
    fn initialized_clip_id_rejects_missing_clip_identity() {
        let response = InitializeAudioClipResponse {
            clip_id: None,
            clip: None,
        };

        let err = initialized_clip_id(&response).expect_err("missing clip id");

        assert_eq!(err.error_code(), "schema_drift");
    }
}
