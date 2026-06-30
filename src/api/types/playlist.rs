use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::clip::Clip;

#[derive(Debug, Deserialize, Serialize)]
pub struct PlaylistListResponse {
    #[serde(default, alias = "numTotalResults")]
    pub num_total_results: u64,
    #[serde(default, alias = "currentPage")]
    pub current_page: u32,
    #[serde(default)]
    pub playlists: Vec<PlaylistInfo>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlaylistInfo {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub image_url: Option<String>,
    #[serde(default)]
    pub is_public: bool,
    #[serde(default)]
    pub is_trashed: bool,
    #[serde(default)]
    pub song_count: Option<u64>,
    #[serde(default, alias = "numTotalResults")]
    pub num_total_results: Option<u64>,
    #[serde(default, alias = "clipIds")]
    pub clip_ids: Vec<String>,
    #[serde(default)]
    pub playlist_clips: Vec<PlaylistClip>,
    #[serde(default, flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

impl PlaylistInfo {
    pub fn clip_count(&self) -> u64 {
        self.song_count
            .or(self.num_total_results)
            .unwrap_or_else(|| self.playlist_clips.len().max(self.clip_ids.len()) as u64)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlaylistClip {
    #[serde(default)]
    pub clip: Option<Clip>,
    #[serde(default)]
    pub relative_index: Option<f64>,
    #[serde(default, flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct CreatePlaylistRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct SetPlaylistMetadataRequest {
    pub playlist_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PlaylistTracksRequest {
    pub clip_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct SetPlaylistVisibilityRequest {
    pub metadata: PlaylistVisibilityMetadata,
}

impl SetPlaylistVisibilityRequest {
    pub fn new(is_public: bool) -> Self {
        Self {
            metadata: PlaylistVisibilityMetadata { is_public },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PlaylistVisibilityMetadata {
    pub is_public: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum PlaylistReaction {
    Like,
    Dislike,
}

impl PlaylistReaction {
    pub fn as_api_value(self) -> &'static str {
        match self {
            Self::Like => "LIKE",
            Self::Dislike => "DISLIKE",
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SetPlaylistReactionRequest {
    pub reaction: Option<String>,
}

impl SetPlaylistReactionRequest {
    pub fn new(reaction: Option<PlaylistReaction>) -> Self {
        Self {
            reaction: reaction.map(|reaction| reaction.as_api_value().to_string()),
        }
    }

    #[cfg(test)]
    pub fn like() -> Self {
        Self::new(Some(PlaylistReaction::Like))
    }

    #[cfg(test)]
    pub fn clear() -> Self {
        Self::new(None)
    }
}

#[derive(Debug, Serialize)]
pub struct PlaylistReorderRequest {
    pub positions: Vec<PlaylistTrackPosition>,
}

impl PlaylistReorderRequest {
    pub fn single(clip_id: impl Into<String>, index: u32) -> Self {
        Self {
            positions: vec![PlaylistTrackPosition {
                clip_id: clip_id.into(),
                index,
            }],
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PlaylistTrackPosition {
    pub clip_id: String,
    pub index: u32,
}

#[derive(Debug, Serialize)]
pub struct TrashPlaylistRequest {
    pub undo: bool,
}

#[cfg(test)]
mod tests {
    use super::{
        CreatePlaylistRequest, PlaylistReorderRequest, PlaylistTracksRequest,
        SetPlaylistMetadataRequest, SetPlaylistReactionRequest, SetPlaylistVisibilityRequest,
        TrashPlaylistRequest,
    };

    #[test]
    fn create_playlist_request_matches_web_shape() {
        let req = CreatePlaylistRequest {
            name: "Mixtape".into(),
        };

        let json = serde_json::to_value(req).expect("serialize request");

        assert_eq!(json, serde_json::json!({ "name": "Mixtape" }));
    }

    #[test]
    fn set_playlist_metadata_omits_absent_fields() {
        let req = SetPlaylistMetadataRequest {
            playlist_id: "playlist-1".into(),
            name: Some("Renamed".into()),
            description: None,
            image_url: None,
        };

        let json = serde_json::to_value(req).expect("serialize request");

        assert_eq!(
            json,
            serde_json::json!({
                "playlist_id": "playlist-1",
                "name": "Renamed"
            })
        );
    }

    #[test]
    fn playlist_track_request_uses_clip_ids() {
        let req = PlaylistTracksRequest {
            clip_ids: vec!["clip-a".into(), "clip-b".into()],
        };

        let json = serde_json::to_value(req).expect("serialize request");

        assert_eq!(
            json,
            serde_json::json!({ "clip_ids": ["clip-a", "clip-b"] })
        );
    }

    #[test]
    fn trash_playlist_request_uses_undo_flag() {
        let req = TrashPlaylistRequest { undo: false };

        let json = serde_json::to_value(req).expect("serialize request");

        assert_eq!(json, serde_json::json!({ "undo": false }));
    }

    #[test]
    fn restore_playlist_request_sets_undo_flag() {
        let req = TrashPlaylistRequest { undo: true };

        let json = serde_json::to_value(req).expect("serialize request");

        assert_eq!(json, serde_json::json!({ "undo": true }));
    }

    #[test]
    fn set_playlist_visibility_uses_v2_metadata_shape() {
        let req = SetPlaylistVisibilityRequest::new(false);

        let json = serde_json::to_value(req).expect("serialize request");

        assert_eq!(
            json,
            serde_json::json!({ "metadata": { "is_public": false } })
        );
    }

    #[test]
    fn reorder_playlist_request_uses_positions_array() {
        let req = PlaylistReorderRequest::single("clip-a", 3);

        let json = serde_json::to_value(req).expect("serialize request");

        assert_eq!(
            json,
            serde_json::json!({ "positions": [{ "clip_id": "clip-a", "index": 3 }] })
        );
    }

    #[test]
    fn set_playlist_metadata_includes_image_url() {
        let req = SetPlaylistMetadataRequest {
            playlist_id: "playlist-1".into(),
            name: None,
            description: None,
            image_url: Some("https://cdn.example/cover.jpg".into()),
        };

        let json = serde_json::to_value(req).expect("serialize request");

        assert_eq!(
            json,
            serde_json::json!({
                "playlist_id": "playlist-1",
                "image_url": "https://cdn.example/cover.jpg"
            })
        );
    }

    #[test]
    fn playlist_like_request_matches_web_shape() {
        let req = SetPlaylistReactionRequest::like();

        let json = serde_json::to_value(req).expect("serialize request");

        assert_eq!(json, serde_json::json!({ "reaction": "LIKE" }));
    }

    #[test]
    fn playlist_clear_reaction_request_matches_web_shape() {
        let req = SetPlaylistReactionRequest::clear();

        let json = serde_json::to_value(req).expect("serialize request");

        assert_eq!(json, serde_json::json!({ "reaction": null }));
    }
}
