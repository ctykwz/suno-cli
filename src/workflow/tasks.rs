use std::time::{Duration, Instant};

use crate::api::SunoClient;
use crate::api::types::Clip;
use crate::core::CliError;

pub fn is_terminal_status(status: &str) -> bool {
    matches!(status, "complete" | "error")
}

pub fn require_found_clips(ids: &[String], clips: Vec<Clip>) -> Result<Vec<Clip>, CliError> {
    let missing = ids
        .iter()
        .filter(|id| !clips.iter().any(|clip| clip.id == **id))
        .cloned()
        .collect::<Vec<_>>();

    if !missing.is_empty() {
        return Err(CliError::NotFound(format!(
            "clip(s): {}",
            missing.join(", ")
        )));
    }

    Ok(clips)
}

pub async fn wait_for_clips(
    client: &SunoClient,
    ids: &[String],
    timeout_secs: u64,
    poll_interval_secs: u64,
) -> Result<Vec<Clip>, CliError> {
    let start = Instant::now();
    let timeout = Duration::from_secs(timeout_secs);
    let mut delay = Duration::from_secs(poll_interval_secs.max(1));

    loop {
        let clips = require_found_clips(ids, client.get_clips(ids).await?)?;
        if clips.iter().all(|clip| is_terminal_status(&clip.status)) {
            return Ok(clips);
        }
        if start.elapsed() >= timeout {
            return Err(CliError::GenerationFailed(format!(
                "generation timed out after {timeout_secs}s for {}",
                ids.join(", ")
            )));
        }
        tokio::time::sleep(delay).await;
        delay = (delay * 2).min(Duration::from_secs(15));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clip(id: &str, status: &str) -> Clip {
        Clip {
            id: id.into(),
            title: format!("Clip {id}"),
            status: status.into(),
            model_name: "chirp-fenix".into(),
            audio_url: None,
            video_url: None,
            image_url: None,
            created_at: "2026-06-30T00:00:00Z".into(),
            play_count: 0,
            upvote_count: 0,
            metadata: Default::default(),
        }
    }

    #[test]
    fn complete_and_error_are_terminal_states() {
        assert!(is_terminal_status("complete"));
        assert!(is_terminal_status("error"));
    }

    #[test]
    fn streaming_and_submitted_are_not_terminal_states() {
        assert!(!is_terminal_status("streaming"));
        assert!(!is_terminal_status("submitted"));
    }

    #[test]
    fn found_clips_rejects_empty_response_for_requested_ids() {
        let ids = vec!["clip-missing".to_string()];

        let err = require_found_clips(&ids, Vec::new()).expect_err("missing clip should fail");

        assert!(matches!(err, CliError::NotFound(message) if message.contains("clip-missing")));
    }

    #[test]
    fn found_clips_rejects_partial_response_for_requested_ids() {
        let ids = vec!["clip-a".to_string(), "clip-b".to_string()];

        let err = require_found_clips(&ids, vec![clip("clip-a", "complete")])
            .expect_err("partial clip response should fail");

        assert!(
            matches!(err, CliError::NotFound(message) if message.contains("clip-b") && !message.contains("clip-a"))
        );
    }

    #[test]
    fn found_clips_returns_complete_response() {
        let ids = vec!["clip-a".to_string(), "clip-b".to_string()];
        let clips = vec![clip("clip-a", "complete"), clip("clip-b", "submitted")];

        let clips = require_found_clips(&ids, clips).expect("all requested clips found");

        assert_eq!(clips.len(), 2);
    }
}
