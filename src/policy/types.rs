use std::{ffi::OsString, path::PathBuf};

pub struct Policy {
    pub workdir: PathBuf,
    pub mounts: Vec<Mount>,
    pub env: Vec<(String, String)>,
    pub command: Vec<OsString>,
}

pub struct Mount {
    pub src: PathBuf,
    pub dst: PathBuf,
    pub kind: MountKind,
}

pub enum MountKind {
    ReadOnly,
    ReadWrite,
    /// An empty in-memory directory (tmpfs) that disappears on exit.
    Tmpfs,
    /// Hidden by mounting `/dev/null` over the path.
    /// The file on the host is untouched.
    Masked,
}
