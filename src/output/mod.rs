//! Output format detection and renderers for JSON and human-readable tables.

pub mod json;
pub mod table;

#[derive(Clone, Copy)]
pub enum OutputFormat {
    Json,
    Table,
}

impl OutputFormat {
    pub fn detect(json_flag: bool) -> Self {
        if json_flag || !std::io::IsTerminal::is_terminal(&std::io::stdout()) {
            OutputFormat::Json
        } else {
            OutputFormat::Table
        }
    }
}
