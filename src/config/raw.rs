use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawConfig {
    pub path: Option<PathBuf>,
    pub level: Option<u8>,
    pub work: Option<Work>,
    pub view: Option<View>,
    pub net: Option<Net>,
    pub git: Option<Git>,
    pub scan: Option<Scan>,
    pub ro: Option<Vec<String>>,
    pub rw: Option<Vec<String>>,
    pub hide: Option<Vec<String>>,
    pub unhide: Option<Vec<String>>,
    pub host: Option<Vec<String>>,
    pub env: Option<Vec<String>>,
    pub add: Option<AddSection>,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AddSection {
    pub ro: Option<Vec<String>>,
    pub rw: Option<Vec<String>>,
    pub hide: Option<Vec<String>>,
    pub unhide: Option<Vec<String>>,
    pub host: Option<Vec<String>>,
    pub env: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Work {
    Dir,
    Tmp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum View {
    All,
    List,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Net {
    Full,
    Allow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Git {
    None,
    Snapshot,
    Full,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Scan {
    Off,
    Warn,
    Hide,
    Abort,
}
