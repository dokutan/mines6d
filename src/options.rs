use serde::{Deserialize, Serialize};
use std::{option::Option, path::PathBuf};

/// Used to pass around global options
pub struct Options {
    pub history_path: Option<PathBuf>,
    pub config_path: Option<PathBuf>,
    pub config: Option<Config>,
}

impl Options {
    pub fn new() -> Self {
        Options {
            history_path: None,
            config_path: None,
            config: None,
        }
    }
}

/// Used to (de)serialize the config file
#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_cells_h_space")]
    pub cells_h_space: usize,
    #[serde(default = "default_use_unicode")]
    pub use_unicode: bool,
    #[serde(default = "default_use_color")]
    pub use_color: bool,
}

// default config values
fn default_cells_h_space() -> usize {
    2
}

fn default_use_unicode() -> bool {
    true
}

fn default_use_color() -> bool {
    true
}
