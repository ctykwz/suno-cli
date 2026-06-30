#[derive(clap::Args)]
pub struct WaitArgs {
    /// Clip ID(s) to wait for
    pub ids: Vec<String>,

    /// Max wait time in seconds
    #[arg(long)]
    pub timeout: Option<u64>,
}
