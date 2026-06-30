use crate::app::AppContext;
use crate::cli::{DownloadArgs, TimedLyricsArgs, UploadArgs};
use crate::core::{CliError, ensure_clip_ids};
use crate::media;
use crate::output::{self, OutputFormat};
use crate::workflow::tasks;
use crate::workflow::upload::{self, UploadWorkflowInput};

pub async fn download(args: DownloadArgs, ctx: &AppContext) -> Result<(), CliError> {
    ensure_clip_ids(&args.ids)?;
    let client = ctx.client().await?;
    let clips = tasks::require_found_clips(&args.ids, client.get_clips(&args.ids).await?)?;
    let mut paths = Vec::new();
    let output_dir = args.output.as_deref().unwrap_or(&ctx.config.output_dir);
    for clip in &clips {
        let path = media::download_clip(clip, output_dir, args.video).await?;

        if !args.video {
            let plain_lyrics = clip.metadata.prompt.as_deref();
            let aligned = client.aligned_lyrics(&clip.id).await.ok();
            media::embed_lyrics_in_mp3(&path, &clip.title, plain_lyrics, aligned.as_deref())?;
            if !ctx.quiet {
                eprintln!("Embedded lyrics into {path}");
            }
        }

        if !ctx.quiet {
            eprintln!("Downloaded: {path}");
        }
        paths.push(path);
    }
    match ctx.fmt {
        OutputFormat::Json => output::json::success(&paths),
        OutputFormat::Table => {}
    }
    Ok(())
}

pub async fn upload(args: UploadArgs, ctx: &AppContext) -> Result<(), CliError> {
    let lyrics = match (&args.lyrics, &args.lyrics_file) {
        (Some(lyrics), _) => Some(lyrics.clone()),
        (_, Some(path)) => Some(std::fs::read_to_string(path)?),
        _ => None,
    };
    let path = std::path::Path::new(&args.file);
    if !ctx.quiet {
        eprintln!("Uploading audio: {}", path.display());
    }

    let client = ctx.client().await?;
    let result = upload::run(
        &client,
        UploadWorkflowInput {
            file: path,
            upload_type: &args.upload_type,
            is_stem_mix: args.stem_mix,
            title: args.title,
            lyrics,
            timeout: std::time::Duration::from_secs(
                args.timeout.unwrap_or(ctx.config.poll_timeout_secs),
            ),
            poll_interval: std::time::Duration::from_secs(ctx.config.poll_interval_secs),
        },
    )
    .await?;

    match ctx.fmt {
        OutputFormat::Json => output::json::success(&result),
        OutputFormat::Table => {
            eprintln!("Upload complete: {}", result.upload_id);
            if let Some(clip_id) = result.clip_id {
                println!("{clip_id}");
            }
        }
    }
    Ok(())
}

pub async fn timed_lyrics(args: TimedLyricsArgs, ctx: &AppContext) -> Result<(), CliError> {
    let words = ctx.client().await?.aligned_lyrics(&args.id).await?;
    match timed_lyrics_render(args.lrc, ctx.fmt) {
        TimedLyricsRender::Json => output::json::success(&words),
        TimedLyricsRender::Lrc => {
            for word in &words {
                if !word.success {
                    continue;
                }
                let mins = (word.start_s / 60.0) as u32;
                let secs = word.start_s % 60.0;
                println!("[{:02}:{:05.2}] {}", mins, secs, word.word);
            }
        }
        TimedLyricsRender::Table => {
            for word in &words {
                if word.success {
                    println!(
                        "{:>6.2}s - {:>6.2}s  {}",
                        word.start_s, word.end_s, word.word
                    );
                }
            }
        }
    }
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum TimedLyricsRender {
    Json,
    Lrc,
    Table,
}

fn timed_lyrics_render(lrc: bool, fmt: OutputFormat) -> TimedLyricsRender {
    match fmt {
        OutputFormat::Json => TimedLyricsRender::Json,
        OutputFormat::Table if lrc => TimedLyricsRender::Lrc,
        OutputFormat::Table => TimedLyricsRender::Table,
    }
}

#[cfg(test)]
mod tests {
    use crate::output::OutputFormat;

    use super::{TimedLyricsRender, timed_lyrics_render};

    #[test]
    fn timed_lyrics_json_output_takes_priority_over_lrc_flag() {
        assert_eq!(
            timed_lyrics_render(true, OutputFormat::Json),
            TimedLyricsRender::Json
        );
    }

    #[test]
    fn timed_lyrics_lrc_applies_to_table_output() {
        assert_eq!(
            timed_lyrics_render(true, OutputFormat::Table),
            TimedLyricsRender::Lrc
        );
    }

    #[test]
    fn timed_lyrics_table_output_is_default_human_format() {
        assert_eq!(
            timed_lyrics_render(false, OutputFormat::Table),
            TimedLyricsRender::Table
        );
    }
}
