use super::CliError;

pub fn ensure_clip_ids(ids: &[String]) -> Result<(), CliError> {
    if ids.is_empty() {
        return Err(CliError::Config("no clip IDs provided".into()));
    }
    Ok(())
}

pub fn ensure_destructive_confirmed(yes: bool, command: &str) -> Result<(), CliError> {
    if !yes {
        return Err(CliError::Config(format!(
            "`{command}` requires -y/--yes because it modifies or removes Suno resources"
        )));
    }
    Ok(())
}
