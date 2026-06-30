use crate::api::types::PersonaInfo;

use super::{base_table, dynamic_table};

pub fn personas(personas: &[PersonaInfo]) {
    let mut table = dynamic_table();
    table.set_header(vec![
        "ID", "Name", "Owner", "Type", "Public", "Trashed", "Loved", "Clips",
    ]);

    for info in personas {
        let short_id = if info.id.len() > 8 {
            &info.id[..8]
        } else {
            &info.id
        };
        let owner = info
            .user_display_name
            .as_deref()
            .or(info.user_handle.as_deref())
            .unwrap_or("-");
        let persona_type = info.persona_type.as_deref().unwrap_or("-");
        table.add_row(vec![
            short_id,
            &info.name,
            owner,
            persona_type,
            &optional_bool(info.is_public),
            &info.is_trashed.to_string(),
            &info.is_loved.to_string(),
            &persona_clip_count(info).to_string(),
        ]);
    }

    println!("{table}");
}

pub fn persona(info: &PersonaInfo) {
    let mut table = base_table();
    table.set_header(vec!["Field", "Value"]);

    table.add_row(vec!["ID", &info.id]);
    table.add_row(vec!["Name", &info.name]);
    table.add_row(vec![
        "Description",
        info.description.as_deref().unwrap_or("-"),
    ]);
    if let Some(ref owner) = info.user_display_name {
        table.add_row(vec!["Owner", owner]);
    }
    if let Some(ref handle) = info.user_handle {
        table.add_row(vec!["Handle", handle]);
    }
    if let Some(ref persona_type) = info.persona_type {
        table.add_row(vec!["Type", persona_type]);
    }
    table.add_row(vec!["Loved", &info.is_loved.to_string()]);
    table.add_row(vec!["Owned", &info.is_owned.to_string()]);
    table.add_row(vec!["Public", &optional_bool(info.is_public)]);
    table.add_row(vec!["Trashed", &info.is_trashed.to_string()]);
    table.add_row(vec!["Hidden", &info.is_hidden.to_string()]);
    table.add_row(vec!["Following", &info.is_following.to_string()]);
    table.add_row(vec!["Clips", &persona_clip_count(info).to_string()]);
    if let Some(follower_count) = info.follower_count {
        table.add_row(vec!["Followers", &follower_count.to_string()]);
    }
    if let Some(ref source) = info.source {
        table.add_row(vec!["Source", source]);
    }
    if let Some(ref styles) = info.user_input_styles {
        table.add_row(vec!["User Styles", styles]);
    }
    if let Some(ref vocal_clip_id) = info.vocal_clip_id {
        table.add_row(vec!["Vocal Clip", vocal_clip_id]);
    }
    if info.vocal_start_s.is_some() || info.vocal_end_s.is_some() {
        table.add_row(vec![
            "Vocal Range",
            &format!(
                "{} - {}",
                info.vocal_start_s
                    .map(|value| format!("{value:.2}s"))
                    .unwrap_or_else(|| "-".into()),
                info.vocal_end_s
                    .map(|value| format!("{value:.2}s"))
                    .unwrap_or_else(|| "-".into())
            ),
        ]);
    }

    println!("{table}");
}

fn optional_bool(value: Option<bool>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "unknown".into())
}

fn persona_clip_count(info: &PersonaInfo) -> u64 {
    info.clip_count.unwrap_or(info.persona_clips.len() as u64)
}
