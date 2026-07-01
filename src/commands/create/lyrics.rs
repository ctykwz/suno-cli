use crate::app::AppContext;
use crate::cli::LyricsArgs;
use crate::core::CliError;
use crate::output::{self, OutputFormat};

pub async fn lyrics(args: LyricsArgs, ctx: &AppContext) -> Result<(), CliError> {
    if !ctx.quiet {
        eprintln!("Generating lyrics...");
    }
    let _mutation_guard = ctx.acquire_mutation_lock()?;
    let result = ctx.client().await?.generate_lyrics(&args.prompt).await?;
    match ctx.fmt {
        OutputFormat::Json => output::json::success(&result),
        OutputFormat::Table => output::table::lyrics(&result),
    }
    Ok(())
}
