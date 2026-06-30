use crate::app::AppContext;
use crate::cli::UpdateArgs;
use crate::core::CliError;
use crate::output::{self, OutputFormat};

pub async fn run(args: UpdateArgs, ctx: &AppContext) -> Result<(), CliError> {
    let current = env!("CARGO_PKG_VERSION");
    let updater = self_update::backends::github::Update::configure()
        .repo_owner("ctykwz")
        .repo_name("sunox")
        .bin_name("sunox")
        .show_download_progress(!ctx.quiet)
        .current_version(current)
        .build()
        .map_err(|e| CliError::Update(e.to_string()))?;

    if args.check {
        let latest = updater
            .get_latest_release()
            .map_err(|e| CliError::Update(e.to_string()))?;
        let v = latest.version.trim_start_matches('v').to_string();
        let up_to_date = v == current;
        let status = if up_to_date {
            "up_to_date"
        } else {
            "update_available"
        };
        let result = serde_json::json!({
            "current_version": current,
            "latest_version": v,
            "status": status,
        });
        match ctx.fmt {
            OutputFormat::Json => output::json::success(&result),
            OutputFormat::Table => {
                if up_to_date {
                    eprintln!("Up to date (v{current})");
                } else {
                    eprintln!("Update available: v{current} -> v{v}");
                    eprintln!("Run `sunox update` to install");
                }
            }
        }
    } else {
        let release = updater
            .update()
            .map_err(|e| CliError::Update(e.to_string()))?;
        let v = release.version().trim_start_matches('v').to_string();
        let up_to_date = v == current;
        let status = if up_to_date { "up_to_date" } else { "updated" };
        let result = serde_json::json!({
            "current_version": current,
            "latest_version": v,
            "status": status,
        });
        match ctx.fmt {
            OutputFormat::Json => output::json::success(&result),
            OutputFormat::Table => {
                if up_to_date {
                    eprintln!("Already up to date (v{current})");
                } else {
                    eprintln!("Updated: v{current} -> v{v}");
                    eprintln!("Run `sunox install-skill --force` to refresh the agent skill");
                }
            }
        }
    }

    Ok(())
}
