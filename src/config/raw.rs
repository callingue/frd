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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_full_config() {
        let raw: RawConfig = toml::from_str(
            r#"
            path = "/home/you/src/app"
            level = 3
            work = "tmp"
            net = "allow"
            host = ["github.com"]

            [add]
            ro = ["~/docs"]
            "#,
        )
        .unwrap();

        assert_eq!(raw.level, Some(3));
        assert_eq!(raw.work, Some(Work::Tmp));
        assert_eq!(raw.net, Some(Net::Allow));
        assert_eq!(raw.add.unwrap().ro, Some(vec!["~/docs".to_string()]));
    }

    #[test]
    fn empty_config_is_valid() {
        let raw: RawConfig = toml::from_str("").unwrap();
        assert!(raw.level.is_none());
    }

    #[test]
    fn rejects_unknown_key() {
        assert!(toml::from_str::<RawConfig>("lvl = 3").is_err());
    }

    #[test]
    fn rejects_unknown_value() {
        assert!(toml::from_str::<RawConfig>(r#"work = "dri""#).is_err());
    }
}
