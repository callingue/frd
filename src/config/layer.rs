use super::raw::RawConfig;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    Default,
    Preset(u8),
    Global(PathBuf),
    Project(PathBuf),
    Repo(PathBuf),
    Cli,
}

pub struct Layer {
    pub source: Source,
    pub config: RawConfig,
}
