use clap::Subcommand;

use super::{
    ConcatArgs, CoverArgs, DeleteArgs, DownloadArgs, ExtendArgs, InfoArgs, ListArgs, PublishArgs,
    ReactionArgs, RemasterArgs, RestoreArgs, SearchArgs, SetArgs, SpeedArgs, StatusArgs, StemsArgs,
    TimedLyricsArgs, UploadArgs, WaitArgs,
};

#[derive(clap::Args)]
pub struct ClipArgs {
    #[command(subcommand)]
    pub command: ClipCommand,
}

#[derive(Subcommand)]
pub enum ClipCommand {
    /// List your songs
    List(ListArgs),

    /// Search your songs by title or tags
    Search(SearchArgs),

    /// Show detailed info for a single clip
    Info(InfoArgs),

    /// Check generation status
    Status(StatusArgs),

    /// Wait for generated clip(s) to finish
    Wait(WaitArgs),

    /// Download audio/video for clip(s)
    Download(DownloadArgs),

    /// Upload a local audio file into your Suno library
    Upload(UploadArgs),

    /// Delete/trash a clip
    Delete(DeleteArgs),

    /// Restore clip(s) from trash
    Restore(RestoreArgs),

    /// Like clip(s), or clear likes with --clear
    Like(ReactionArgs),

    /// Dislike clip(s), or clear dislikes with --clear
    Dislike(ReactionArgs),

    /// Update clip title, lyrics, or caption
    Set(SetArgs),

    /// Toggle clip public/private
    Publish(PublishArgs),

    /// Get word-level timestamped lyrics
    TimedLyrics(TimedLyricsArgs),

    /// Continue/extend a clip from a timestamp
    Extend(ExtendArgs),

    /// Concatenate clips into a full song
    Concat(ConcatArgs),

    /// Create a cover of an existing clip
    Cover(CoverArgs),

    /// Remaster a clip with a different model
    Remaster(RemasterArgs),

    /// Adjust playback speed for a clip
    Speed(SpeedArgs),

    /// Extract stems (vocals, instruments) from a clip
    Stems(StemsArgs),
}
