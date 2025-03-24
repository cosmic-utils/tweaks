use std::path::PathBuf;

use cosmic::{
    cosmic_config::{self, Config},
    cosmic_theme::ThemeBuilder,
};
use cosmic_config::cosmic_config_derive::CosmicConfigEntry;
use cosmic_config::CosmicConfigEntry;
use serde::{Deserialize, Serialize};

const CONFIG_ID: &str = "dev.edfloreshz.CosmicTweaks.ColorScheme";

#[derive(Debug, Serialize, Clone, Default, Deserialize, PartialEq, CosmicConfigEntry)]
#[version = 1]
pub struct ColorScheme {
    pub name: String,
    pub path: Option<PathBuf>,
    pub link: Option<String>,
    pub author: Option<String>,
    pub theme: ThemeBuilder,
}

impl ColorScheme {
    #[allow(dead_code)]
    pub const fn version() -> u64 {
        Self::VERSION
    }

    /// Get the config for the theme
    pub fn config() -> Config {
        match Config::new(CONFIG_ID, Self::VERSION) {
            Ok(config) => config,
            Err(err) => panic!("Failed to load config: {}", err),
        }
    }

    pub fn theme(&self) -> anyhow::Result<ThemeBuilder> {
        let Some(path) = self.path.as_ref() else {
            anyhow::bail!("No path for the theme")
        };

        let file = std::fs::read_to_string(path)?;
        let theme: ThemeBuilder = ron::from_str(&file)?;
        Ok(theme)
    }
}
