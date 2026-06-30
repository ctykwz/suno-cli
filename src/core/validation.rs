use super::CliError;

pub fn ensure_clip_ids(ids: &[String]) -> Result<(), CliError> {
    if ids.is_empty() {
        return Err(CliError::Config("no clip IDs provided".into()));
    }
    Ok(())
}
