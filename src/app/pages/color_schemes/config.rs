use cosmic::cosmic_config::{self, Config};
use cosmic_config::CosmicConfigEntry;
use cosmic_config::cosmic_config_derive::CosmicConfigEntry;
use serde::{Deserialize, Serialize};

use crate::app::pages::color_schemes::models::ColorScheme;

#[derive(Debug, Serialize, Clone, Default, Deserialize, PartialEq, CosmicConfigEntry)]
#[version = 1]
pub struct ColorSchemesPageConfig {
    pub current_config: Option<ColorScheme>,
}

const CONFIG_ID: &str = "dev.edfloreshz.CosmicTweaks.ColorScheme";

impl ColorSchemesPageConfig {
    pub fn config() -> Config {
        match Config::new(CONFIG_ID, Self::VERSION) {
            Ok(config) => config,
            Err(err) => panic!("Failed to load config: {}", err),
        }
    }
}
