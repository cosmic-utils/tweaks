use cosmic::{
    Application,
    cosmic_config::{self, Config, CosmicConfigEntry, cosmic_config_derive::CosmicConfigEntry},
    theme,
};
use serde::{Deserialize, Serialize};

use crate::app::App;

pub const CONFIG_VERSION: u64 = 1;

#[derive(Clone, Default, Debug, Eq, PartialEq, Deserialize, Serialize, CosmicConfigEntry)]
pub struct TweaksConfig {
    pub app_theme: AppTheme,
}

impl TweaksConfig {
    pub fn config() -> Config {
        match Config::new(App::APP_ID, CONFIG_VERSION) {
            Ok(config) => config,
            Err(err) => panic!("Failed to fetch config for application: {err}"),
        }
    }

    pub fn new() -> TweaksConfig {
        TweaksConfig::get_entry(&Self::config()).unwrap_or_else(|(errs, config)| {
            log::info!("errors loading config: {:?}", errs);
            config
        })
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum AppTheme {
    Dark,
    Light,
    #[default]
    System,
}

impl AppTheme {
    pub fn theme(&self) -> theme::Theme {
        match self {
            Self::Dark => {
                let mut t = theme::system_dark();
                t.theme_type.prefer_dark(Some(true));
                t
            }
            Self::Light => {
                let mut t = theme::system_light();
                t.theme_type.prefer_dark(Some(false));
                t
            }
            Self::System => theme::system_preference(),
        }
    }
}
