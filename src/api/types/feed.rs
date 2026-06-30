use serde::{Deserialize, Serialize};

use super::clip::Clip;

#[derive(Debug, Deserialize, Serialize)]
pub struct FeedResponse {
    #[serde(default)]
    pub clips: Vec<Clip>,
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub has_more: bool,
}

#[derive(Debug, Serialize)]
pub struct FeedV3Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FeedFilters>,
}

#[derive(Debug, Serialize)]
pub struct FeedFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "searchText")]
    pub search_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disliked: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trashed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fullSong")]
    pub full_song: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fromStudioProject")]
    pub from_studio_project: Option<FilterPresence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stem: Option<FilterPresence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<WorkspaceFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserFilter>,
}

#[derive(Debug, Serialize)]
pub struct FilterPresence {
    pub presence: String,
}

#[derive(Debug, Serialize)]
pub struct WorkspaceFilter {
    pub presence: String,
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
}

#[derive(Debug, Serialize)]
pub struct UserFilter {
    pub presence: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}

impl FeedFilters {
    pub fn default_workspace() -> Self {
        Self {
            search_text: None,
            disliked: Some("False".to_string()),
            trashed: Some("False".to_string()),
            full_song: None,
            from_studio_project: Some(FilterPresence::absent()),
            stem: Some(FilterPresence::absent()),
            workspace: Some(WorkspaceFilter::default_workspace()),
            user: None,
        }
    }

    pub fn search(query: &str) -> Self {
        Self {
            search_text: Some(query.to_string()),
            ..Self::default_workspace()
        }
    }
}

impl FilterPresence {
    pub fn absent() -> Self {
        Self {
            presence: "False".to_string(),
        }
    }
}

impl WorkspaceFilter {
    pub fn default_workspace() -> Self {
        Self {
            presence: "True".to_string(),
            workspace_id: "default".to_string(),
        }
    }
}

#[cfg(test)]
impl UserFilter {
    pub fn for_user(user_id: impl Into<String>) -> Self {
        Self {
            presence: "True".to_string(),
            user_id: user_id.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FeedFilters, FeedV3Request, UserFilter};

    #[test]
    fn default_feed_matches_create_page_workspace_filter() {
        let req = FeedV3Request {
            cursor: None,
            limit: Some(20),
            filters: Some(FeedFilters::default_workspace()),
        };

        let json = serde_json::to_value(req).expect("serialize feed request");

        assert_eq!(json["cursor"], serde_json::Value::Null);
        assert_eq!(json["limit"], 20);
        assert_eq!(json["filters"]["disliked"], "False");
        assert_eq!(json["filters"]["trashed"], "False");
        assert_eq!(json["filters"]["fromStudioProject"]["presence"], "False");
        assert_eq!(json["filters"]["stem"]["presence"], "False");
        assert_eq!(json["filters"]["workspace"]["presence"], "True");
        assert_eq!(json["filters"]["workspace"]["workspaceId"], "default");
    }

    #[test]
    fn user_feed_filter_matches_me_page_shape() {
        let mut filters = FeedFilters::default_workspace();
        filters.workspace = None;
        filters.user = Some(UserFilter::for_user("user-123"));

        let req = FeedV3Request {
            cursor: None,
            limit: Some(20),
            filters: Some(filters),
        };

        let json = serde_json::to_value(req).expect("serialize user feed request");

        assert_eq!(json["filters"]["user"]["presence"], "True");
        assert_eq!(json["filters"]["user"]["userId"], "user-123");
        assert!(json["filters"].get("workspace").is_none());
    }
}
