#[derive(clap::Args)]
pub struct UpdateArgs {
    /// Check for a new version without installing
    #[arg(long)]
    pub check: bool,
}
