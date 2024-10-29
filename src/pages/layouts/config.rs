use std::path::PathBuf;

use cosmic::{
    cosmic_config::{self, cosmic_config_derive::CosmicConfigEntry, Config, CosmicConfigEntry},
    Application,
};
use cosmic_ext_config_templates::Schema;
use serde::{Deserialize, Serialize};

use crate::app::TweakTool;

const COSMIC: &str = include_str!("../layouts/files/cosmic.ron");
const MAC: &str = include_str!("../layouts/files/mac.ron");
const WINDOWS: &str = include_str!("../layouts/files/windows.ron");
const UBUNTU: &str = include_str!("../layouts/files/ubuntu.ron");

#[derive(Debug, Serialize, Clone, Deserialize, PartialEq, CosmicConfigEntry)]
#[version = 1]
pub struct LayoutsConfig {
    pub layouts: Vec<Layout>,
}

impl Default for LayoutsConfig {
    fn default() -> Self {
        Self {
            layouts: vec![Layout::Cosmic, Layout::Mac, Layout::Windows, Layout::Ubuntu],
        }
    }
}

impl LayoutsConfig {
    pub fn helper() -> Option<Config> {
        Config::new(TweakTool::APP_ID, Self::VERSION).ok()
    }

    pub fn config() -> LayoutsConfig {
        match Self::helper() {
            Some(config_handler) => {
                LayoutsConfig::get_entry(&config_handler).unwrap_or_else(|(errs, config)| {
                    log::info!("errors loading config: {:?}", errs);
                    config
                })
            }
            None => LayoutsConfig::default(),
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize, PartialEq)]
pub enum Layout {
    Cosmic,
    Mac,
    Windows,
    Ubuntu,
    Custom(CustomLayout),
}

impl Layout {
    pub fn name(&self) -> &str {
        match self {
            Layout::Cosmic => "cosmic",
            Layout::Mac => "mac",
            Layout::Windows => "windows",
            Layout::Ubuntu => "ubuntu",
            Layout::Custom(custom_layout) => &custom_layout.name,
        }
    }

    pub fn schema(&self) -> Schema {
        match self {
            Layout::Cosmic => ron::from_str::<Schema>(COSMIC).unwrap(),
            Layout::Mac => ron::from_str::<Schema>(MAC).unwrap(),
            Layout::Windows => ron::from_str::<Schema>(WINDOWS).unwrap(),
            Layout::Ubuntu => ron::from_str::<Schema>(UBUNTU).unwrap(),
            Layout::Custom(custom_layout) => Schema::from_file(&custom_layout.path).unwrap(),
        }
    }
}

#[derive(Debug, Serialize, Clone, Default, Deserialize, PartialEq, CosmicConfigEntry)]
pub struct CustomLayout {
    name: String,
    path: PathBuf,
}

impl CustomLayout {
    pub fn new(name: String, path: &PathBuf) -> Self {
        Self {
            name,
            path: path.clone(),
        }
    }
}
