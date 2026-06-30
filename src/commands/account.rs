use crate::app::AppContext;
use crate::core::CliError;
use crate::output::{self, OutputFormat};

pub async fn credits(ctx: &AppContext) -> Result<(), CliError> {
    let info = ctx.client().await?.billing_info().await?;
    match ctx.fmt {
        OutputFormat::Json => output::json::success(&info),
        OutputFormat::Table => output::table::billing(&info),
    }
    Ok(())
}

pub async fn models(ctx: &AppContext) -> Result<(), CliError> {
    let info = ctx.client().await?.billing_info().await?;
    match ctx.fmt {
        OutputFormat::Json => output::json::success(&info.models),
        OutputFormat::Table => output::table::models(&info.models),
    }
    Ok(())
}
