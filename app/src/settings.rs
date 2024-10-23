use cosmic::{
    cosmic_config::{self, cosmic_config_derive::CosmicConfigEntry, Config, CosmicConfigEntry},
    theme, Application,
};
use serde::{Deserialize, Serialize};

use crate::app::TweakTool;

pub const CONFIG_VERSION: u64 = 1;

#[derive(Clone, Default, Debug, Eq, PartialEq, Deserialize, Serialize, CosmicConfigEntry)]
pub struct TweaksSettings {
    pub app_theme: AppTheme,
    pub favorites: Vec<Tweak>,
}

impl TweaksSettings {
    pub fn config_handler() -> Option<Config> {
        Config::new(TweakTool::APP_ID, CONFIG_VERSION).ok()
    }

    pub fn config() -> TweaksSettings {
        match Self::config_handler() {
            Some(config_handler) => {
                TweaksSettings::get_entry(&config_handler).unwrap_or_else(|(errs, config)| {
                    log::info!("errors loading config: {:?}", errs);
                    config
                })
            }
            None => TweaksSettings::default(),
        }
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Tweak {
    DockPadding,
    DockSpacing,
    PanelShow,
    PanelForceIcons,
    PanelPadding,
    PanelSpacing,
}
