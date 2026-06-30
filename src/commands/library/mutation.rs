use serde_json::{Value, json};

use crate::api::types::{ClipReaction, SetMetadataRequest};
use crate::app::AppContext;
use crate::cli::{DeleteArgs, PublishArgs, ReactionArgs, RestoreArgs, SetArgs};
use crate::core::{CliError, ensure_clip_ids};
use crate::output::{self, OutputFormat};

pub async fn delete(args: DeleteArgs, ctx: &AppContext) -> Result<(), CliError> {
    ensure_clip_ids(&args.ids)?;
    if !args.yes {
        eprintln!(
            "Deleting {} clip(s): {}",
            args.ids.len(),
            args.ids.join(", ")
        );
        eprintln!("Use -y to skip confirmation, or press Ctrl+C to cancel");
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
    ctx.client().await?.delete_clips(&args.ids).await?;
    match ctx.fmt {
        OutputFormat::Json => output::json::success(clip_ids_result(&args.ids, "deleted", true)),
        OutputFormat::Table => eprintln!("Deleted {} clip(s)", args.ids.len()),
    }
    Ok(())
}

pub async fn restore(args: RestoreArgs, ctx: &AppContext) -> Result<(), CliError> {
    ensure_clip_ids(&args.ids)?;
    ctx.client().await?.restore_clips(&args.ids).await?;
    match ctx.fmt {
        OutputFormat::Json => output::json::success(clip_ids_result(&args.ids, "restored", true)),
        OutputFormat::Table => eprintln!("Restored {} clip(s)", args.ids.len()),
    }
    Ok(())
}

pub async fn like(args: ReactionArgs, ctx: &AppContext) -> Result<(), CliError> {
    react(args, ctx, ClipReaction::Like).await
}

pub async fn dislike(args: ReactionArgs, ctx: &AppContext) -> Result<(), CliError> {
    react(args, ctx, ClipReaction::Dislike).await
}

pub async fn set(args: SetArgs, ctx: &AppContext) -> Result<(), CliError> {
    let changes = set_changed_fields(&args);
    if changes.is_empty() {
        return Err(CliError::Config(
            "provide at least one metadata field: --title, --lyrics, --lyrics-file, --caption, or --remove-cover".into(),
        ));
    }

    let lyrics = match (&args.lyrics, &args.lyrics_file) {
        (Some(l), _) => Some(l.clone()),
        (_, Some(path)) => Some(std::fs::read_to_string(path)?),
        _ => None,
    };
    let req = SetMetadataRequest {
        title: args.title.clone(),
        lyrics,
        caption: args.caption.clone(),
        image_url: None,
        is_audio_upload_tos_accepted: None,
        remove_image_cover: if args.remove_cover { Some(true) } else { None },
        remove_video_cover: None,
    };
    ctx.client().await?.set_metadata(&args.id, &req).await?;
    match ctx.fmt {
        OutputFormat::Json => output::json::success(set_result(&args.id, &changes)),
        OutputFormat::Table => eprintln!("Updated: {}", changes.join(", ")),
    }
    Ok(())
}

pub async fn publish(args: PublishArgs, ctx: &AppContext) -> Result<(), CliError> {
    ensure_clip_ids(&args.ids)?;
    let client = ctx.client().await?;
    let is_public = !args.private;
    for id in &args.ids {
        client.set_visibility(id, is_public).await?;
    }
    let state = if is_public { "public" } else { "private" };
    match ctx.fmt {
        OutputFormat::Json => output::json::success(json!({
            "clip_ids": args.ids,
            "is_public": is_public
        })),
        OutputFormat::Table => eprintln!("Set {} clip(s) to {state}", args.ids.len()),
    }
    Ok(())
}

async fn react(
    args: ReactionArgs,
    ctx: &AppContext,
    reaction: ClipReaction,
) -> Result<(), CliError> {
    ensure_clip_ids(&args.ids)?;
    let client = ctx.client().await?;
    let next_reaction = if args.clear { None } else { Some(reaction) };
    for id in &args.ids {
        client.set_clip_reaction(id, next_reaction).await?;
    }
    let action = match (reaction, args.clear) {
        (ClipReaction::Like, false) => "Liked",
        (ClipReaction::Like, true) => "Cleared like for",
        (ClipReaction::Dislike, false) => "Disliked",
        (ClipReaction::Dislike, true) => "Cleared dislike for",
    };
    match ctx.fmt {
        OutputFormat::Json => {
            output::json::success(reaction_result(&args.ids, reaction, args.clear))
        }
        OutputFormat::Table => eprintln!("{action} {} clip(s)", args.ids.len()),
    }
    Ok(())
}

fn clip_ids_result(ids: &[String], key: &str, value: bool) -> Value {
    let mut result = serde_json::Map::new();
    result.insert("clip_ids".to_string(), json!(ids));
    result.insert(key.to_string(), json!(value));
    Value::Object(result)
}

fn reaction_result(ids: &[String], reaction: ClipReaction, cleared: bool) -> Value {
    json!({
        "clip_ids": ids,
        "reaction": reaction.as_api_value(),
        "cleared": cleared
    })
}

fn set_changed_fields(args: &SetArgs) -> Vec<&'static str> {
    let mut changes = Vec::new();
    if args.title.is_some() {
        changes.push("title");
    }
    if args.lyrics.is_some() || args.lyrics_file.is_some() {
        changes.push("lyrics");
    }
    if args.caption.is_some() {
        changes.push("caption");
    }
    if args.remove_cover {
        changes.push("cover");
    }
    changes
}

fn set_result(clip_id: &str, changes: &[&str]) -> Value {
    json!({
        "clip_id": clip_id,
        "updated": changes
    })
}

#[cfg(test)]
mod tests {
    use super::{clip_ids_result, reaction_result, set_result};
    use crate::api::types::ClipReaction;

    #[test]
    fn delete_result_reports_deleted_clip_ids() {
        let ids = vec!["clip-a".to_string(), "clip-b".to_string()];

        let value = clip_ids_result(&ids, "deleted", true);

        assert_eq!(
            value,
            serde_json::json!({
                "clip_ids": ["clip-a", "clip-b"],
                "deleted": true
            })
        );
    }

    #[test]
    fn reaction_result_reports_clear_state() {
        let ids = vec!["clip-a".to_string()];

        let value = reaction_result(&ids, ClipReaction::Dislike, true);

        assert_eq!(
            value,
            serde_json::json!({
                "clip_ids": ["clip-a"],
                "reaction": "DISLIKE",
                "cleared": true
            })
        );
    }

    #[test]
    fn set_result_reports_changed_fields() {
        let value = set_result("clip-a", &["title", "lyrics"]);

        assert_eq!(
            value,
            serde_json::json!({
                "clip_id": "clip-a",
                "updated": ["title", "lyrics"]
            })
        );
    }
}
