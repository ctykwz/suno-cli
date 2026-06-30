use clap::ValueEnum;

#[derive(clap::Args)]
pub struct InstallSkillArgs {
    /// Target coding agent
    #[arg(long, default_value = "codex")]
    pub target: SkillTarget,

    /// Custom output path (overrides --target default)
    #[arg(long)]
    pub path: Option<String>,

    /// Overwrite existing skill file
    #[arg(short, long)]
    pub force: bool,

    /// Print the skill content to stdout instead of writing
    #[arg(long)]
    pub print: bool,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum SkillTarget {
    /// Codex / Trae CLI (~/.codex/skills/sunox/SKILL.md)
    Codex,
    /// Claude Code (~/.claude/skills/sunox/SKILL.md)
    Claude,
    /// Cursor (.cursor/rules/sunox.mdc in current dir)
    Cursor,
}
