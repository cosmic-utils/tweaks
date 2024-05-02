use cosmic::cosmic_config::{self, Config};
use cosmic_config::cosmic_config_derive::CosmicConfigEntry;
use cosmic_config::CosmicConfigEntry;
use serde::{Deserialize, Serialize};

pub const COLOR_SCHEME_CONFIG_ID: &str = "dev.edfloreshz.CosmicTweakTool.ColorScheme";

#[derive(Debug, Serialize, Clone, Default, Deserialize, CosmicConfigEntry)]
#[version = 1]
pub struct ColorScheme {
    pub name: String,
}

impl ColorScheme {
    pub const fn version() -> u64 {
        Self::VERSION
    }

    /// Get the config for the theme
    pub fn config() -> Result<Config, cosmic_config::Error> {
        Config::new(COLOR_SCHEME_CONFIG_ID, Self::VERSION)
    }
}
