use crate::api::types::Clip;

use super::{base_table, dynamic_table};

pub fn clips(clips: &[Clip]) {
    let mut table = dynamic_table();
    table.set_header(vec!["ID", "Title", "Status", "Model", "Duration", "Tags"]);

    for clip in clips {
        let duration = clip
            .metadata
            .duration
            .map(|duration| format!("{duration:.0}s"))
            .unwrap_or_default();
        let tags = clip.metadata.tags.as_deref().unwrap_or("-");
        let short_id = if clip.id.len() > 8 {
            &clip.id[..8]
        } else {
            &clip.id
        };
        table.add_row(vec![
            short_id,
            &clip.title,
            &clip.status,
            &clip.model_name,
            &duration,
            tags,
        ]);
    }
    println!("{table}");
}

pub fn clip_detail(clip: &Clip) {
    let mut table = base_table();
    table.set_header(vec!["Field", "Value"]);

    table.add_row(vec!["ID", &clip.id]);
    table.add_row(vec!["Title", &clip.title]);
    table.add_row(vec!["Status", &clip.status]);
    table.add_row(vec!["Model", &clip.model_name]);
    table.add_row(vec!["Created", &clip.created_at]);
    table.add_row(vec![
        "Duration",
        &clip
            .metadata
            .duration
            .map(|duration| format!("{duration:.1}s"))
            .unwrap_or_else(|| "-".into()),
    ]);
    table.add_row(vec!["Tags", clip.metadata.tags.as_deref().unwrap_or("-")]);
    table.add_row(vec![
        "BPM",
        &clip
            .metadata
            .avg_bpm
            .map(|bpm| format!("{bpm:.0}"))
            .unwrap_or_else(|| "-".into()),
    ]);
    table.add_row(vec!["Plays", &clip.play_count.to_string()]);
    table.add_row(vec!["Upvotes", &clip.upvote_count.to_string()]);
    table.add_row(vec!["Has Stems", &clip.metadata.has_stem.to_string()]);
    table.add_row(vec![
        "Instrumental",
        &clip.metadata.make_instrumental.to_string(),
    ]);

    if let Some(ref url) = clip.audio_url {
        table.add_row(vec!["Audio URL", url]);
    }
    if let Some(ref url) = clip.video_url {
        table.add_row(vec!["Video URL", url]);
    }
    if let Some(ref prompt) = clip.metadata.prompt {
        let truncated = if prompt.len() > 200 {
            format!("{}...", &prompt[..200])
        } else {
            prompt.clone()
        };
        table.add_row(vec!["Lyrics", &truncated]);
    }

    println!("{table}");
}
