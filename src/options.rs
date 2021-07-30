use std::{option::Option, path::PathBuf};

/// Used to pass around global options
pub struct Options {
    pub history_path: Option<PathBuf>,
    pub config_path: Option<PathBuf>,
}

impl Options {
    pub fn new() -> Self {
        Options {
            history_path: None,
            config_path: None,
        }
    }
}
