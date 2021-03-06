use serde::{Deserialize, Serialize};
use std::{option::Option, path::PathBuf};

/// Used to pass around global options
pub struct Options {
    pub history_path: Option<PathBuf>,
    pub config_path: Option<PathBuf>,
    pub config: Option<Config>,
}

impl Options {
    pub const fn new() -> Self {
        Self {
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
    #[serde(default = "default_theme_file")]
    pub theme_file: String,
}

// default config values
const fn default_cells_h_space() -> usize {
    2
}

const fn default_use_unicode() -> bool {
    true
}

const fn default_use_color() -> bool {
    true
}

fn default_theme_file() -> String {
    "".to_string()
}
