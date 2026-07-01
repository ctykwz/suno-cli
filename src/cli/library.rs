use std::path::PathBuf;

#[derive(clap::Args)]
pub struct InfoArgs {
    /// Clip ID to inspect
    pub id: String,
}

#[derive(clap::Args)]
pub struct PersonaArgs {
    /// Persona ID to view
    pub id: String,
}

#[derive(clap::Args)]
pub struct ListArgs {
    /// Cursor returned by the previous feed response
    #[arg(long)]
    pub cursor: Option<String>,
}

#[derive(clap::Args)]
pub struct SearchArgs {
    /// Search query (matches title and tags)
    pub query: String,
}

#[derive(clap::Args)]
pub struct DeleteArgs {
    /// Clip ID(s) to delete
    pub ids: Vec<String>,

    /// Confirm this destructive action
    #[arg(short = 'y', long)]
    pub yes: bool,
}

#[derive(clap::Args)]
pub struct RestoreArgs {
    /// Clip ID(s) to restore from trash
    pub ids: Vec<String>,
}

#[derive(clap::Args)]
pub struct ReactionArgs {
    /// Clip ID(s) to update
    pub ids: Vec<String>,

    /// Clear this reaction instead of setting it
    #[arg(long)]
    pub clear: bool,
}

#[derive(clap::Args)]
pub struct StatusArgs {
    /// Clip ID(s) to check
    pub ids: Vec<String>,
}

#[derive(clap::Args)]
pub struct SetArgs {
    /// Clip ID to update
    pub id: String,

    /// New title
    #[arg(long)]
    pub title: Option<String>,

    /// New lyrics text
    #[arg(long)]
    pub lyrics: Option<String>,

    /// Read lyrics from file
    #[arg(long)]
    pub lyrics_file: Option<String>,

    /// New caption
    #[arg(long)]
    pub caption: Option<String>,

    /// New clip cover image URL
    #[arg(long, conflicts_with_all = ["image_file", "remove_cover"])]
    pub image_url: Option<String>,

    /// Local image file to upload and use as clip cover
    #[arg(long, conflicts_with_all = ["image_url", "remove_cover"])]
    pub image_file: Option<PathBuf>,

    /// Remove custom cover image
    #[arg(long)]
    pub remove_cover: bool,

    /// Remove custom video cover
    #[arg(long)]
    pub remove_video_cover: bool,
}

#[derive(clap::Args)]
pub struct PublishArgs {
    /// Clip ID(s)
    pub ids: Vec<String>,

    /// Make public (default) or --private
    #[arg(long)]
    pub private: bool,
}
