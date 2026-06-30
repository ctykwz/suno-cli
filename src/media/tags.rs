use id3::TagLike;

use crate::api::types::AlignedWord;
use crate::core::CliError;

/// Embed lyrics and metadata into an MP3 file using ID3v2 tags.
/// - USLT: unsynchronized lyrics, shown in most players
/// - SYLT: synchronized lyrics with word timestamps
/// - TIT2: title
pub fn embed_lyrics_in_mp3(
    mp3_path: &str,
    title: &str,
    plain_lyrics: Option<&str>,
    aligned_words: Option<&[AlignedWord]>,
) -> Result<(), CliError> {
    let mut tag = id3::Tag::read_from_path(mp3_path).unwrap_or_else(|_| id3::Tag::new());
    tag.set_title(title);

    if let Some(lyrics) = plain_lyrics {
        tag.add_frame(id3::frame::Lyrics {
            lang: "eng".to_string(),
            description: String::new(),
            text: lyrics.to_string(),
        });
    }

    if let Some(words) = aligned_words {
        let content: Vec<(u32, String)> = words
            .iter()
            .filter(|w| w.success)
            .map(|w| ((w.start_s * 1000.0) as u32, w.word.clone()))
            .collect();

        if !content.is_empty() {
            tag.add_frame(id3::frame::SynchronisedLyrics {
                lang: "eng".to_string(),
                timestamp_format: id3::frame::TimestampFormat::Ms,
                content_type: id3::frame::SynchronisedLyricsType::Lyrics,
                description: String::new(),
                content,
            });
        }
    }

    tag.write_to_path(mp3_path, id3::Version::Id3v24)
        .map_err(|e| CliError::Download(format!("failed to write ID3 tags: {e}")))?;

    Ok(())
}
