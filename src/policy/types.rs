use std::{ffi::OsString, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Policy {
    pub workdir: PathBuf,
    pub mounts: Vec<Mount>,
    pub env: Vec<(String, String)>,
    pub command: Vec<OsString>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mount {
    Bind {
        src: PathBuf,
        dst: PathBuf,
        writable: bool,
        optional: bool,
    },
    Symlink {
        target: PathBuf,
        dst: PathBuf,
    },
    Tmpfs {
        dst: PathBuf,
    },
    Masked {
        dst: PathBuf,
    },
}
