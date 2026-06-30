use std::path::Path;

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};

use crate::api::types::Clip;
use crate::core::CliError;
use crate::net::http;

pub async fn download_clip(clip: &Clip, output_dir: &str, video: bool) -> Result<String, CliError> {
    let url = if video {
        clip.video_url
            .as_deref()
            .ok_or_else(|| CliError::Download("no video URL available".into()))?
    } else {
        clip.audio_url
            .as_deref()
            .ok_or_else(|| CliError::Download("no audio URL available".into()))?
    };

    let ext = if video { "mp4" } else { "mp3" };
    let slug: String = clip
        .title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .replace("--", "-")
        .trim_matches('-')
        .to_string();
    let short_id = &clip.id[..8.min(clip.id.len())];
    let filename = format!("{slug}-{short_id}.{ext}");
    let path = Path::new(output_dir).join(&filename);

    let resp = http::default_client()?
        .get(url)
        .send()
        .await
        .map_err(CliError::Http)?
        .error_for_status()
        .map_err(CliError::Http)?;

    let total = resp.content_length().unwrap_or(0);
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40}] {bytes}/{total_bytes} ({eta})")
            .unwrap_or_else(|_| ProgressStyle::default_bar())
            .progress_chars("=> "),
    );
    pb.set_message(filename.clone());

    let mut file = tokio::fs::File::create(&path).await?;
    let mut stream = resp.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(CliError::Http)?;
        pb.inc(chunk.len() as u64);
        tokio::io::AsyncWriteExt::write_all(&mut file, &chunk).await?;
    }
    pb.finish_with_message("done");

    Ok(path.display().to_string())
}
