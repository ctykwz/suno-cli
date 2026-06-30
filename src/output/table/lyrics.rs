use crate::api::types::LyricsResult;

pub fn lyrics(result: &LyricsResult) {
    println!("Title: {}\n", result.title);
    println!("{}", result.text);
    if !result.tags.is_empty() {
        println!("\nSuggested style: {}", result.tags.join(", "));
    }
}
