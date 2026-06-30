use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ClipTrashRequest {
    pub trash: bool,
    pub clip_ids: Vec<String>,
}

#[derive(Clone, Copy, Debug)]
pub enum ClipReaction {
    Like,
    Dislike,
}

impl ClipReaction {
    pub fn as_api_value(self) -> &'static str {
        match self {
            Self::Like => "LIKE",
            Self::Dislike => "DISLIKE",
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SetClipReactionRequest {
    pub reaction: Option<String>,
    pub recommendation_metadata: RecommendationMetadata,
}

impl SetClipReactionRequest {
    pub fn new(reaction: Option<ClipReaction>) -> Self {
        Self {
            reaction: reaction.map(|reaction| reaction.as_api_value().to_string()),
            recommendation_metadata: RecommendationMetadata::default(),
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct RecommendationMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_item_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{ClipReaction, ClipTrashRequest, SetClipReactionRequest};

    #[test]
    fn clip_trash_request_matches_web_shape() {
        let req = ClipTrashRequest {
            trash: true,
            clip_ids: vec!["clip-a".into(), "clip-b".into()],
        };

        let json = serde_json::to_value(req).expect("serialize request");

        assert_eq!(
            json,
            serde_json::json!({
                "trash": true,
                "clip_ids": ["clip-a", "clip-b"]
            })
        );
    }

    #[test]
    fn clip_restore_request_matches_web_shape() {
        let req = ClipTrashRequest {
            trash: false,
            clip_ids: vec!["clip-a".into()],
        };

        let json = serde_json::to_value(req).expect("serialize request");

        assert_eq!(
            json,
            serde_json::json!({
                "trash": false,
                "clip_ids": ["clip-a"]
            })
        );
    }

    #[test]
    fn clip_like_request_matches_web_shape() {
        let req = SetClipReactionRequest::new(Some(ClipReaction::Like));

        let json = serde_json::to_value(req).expect("serialize request");

        assert_eq!(
            json,
            serde_json::json!({
                "reaction": "LIKE",
                "recommendation_metadata": {}
            })
        );
    }

    #[test]
    fn clip_clear_reaction_request_matches_web_shape() {
        let req = SetClipReactionRequest::new(None);

        let json = serde_json::to_value(req).expect("serialize request");

        assert_eq!(
            json,
            serde_json::json!({
                "reaction": null,
                "recommendation_metadata": {}
            })
        );
    }
}
