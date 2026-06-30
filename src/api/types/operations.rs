use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ConcatRequest {
    pub clip_id: String,
}
