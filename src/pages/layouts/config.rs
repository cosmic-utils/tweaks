use std::path::PathBuf;

use super::{
    factory::{LayoutPreview, PanelProperties, Position},
    Message,
};
use crate::{app::TweakTool, resources};
use cosmic::{
    cosmic_config::{self, cosmic_config_derive::CosmicConfigEntry, Config, CosmicConfigEntry},
    widget, Application, Element,
};
use cosmic_ext_config_templates::Schema;
use serde::{Deserialize, Serialize};

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
    pub fn file_name(&self) -> &str {
        match self {
            Layout::Cosmic => "cosmic",
            Layout::Mac => "mac",
            Layout::Windows => "windows",
            Layout::Ubuntu => "ubuntu",
            Layout::Custom(custom_layout) => &custom_layout.name,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Layout::Cosmic => "COSMIC",
            Layout::Mac => "macOS",
            Layout::Windows => "Windows",
            Layout::Ubuntu => "Ubuntu",
            Layout::Custom(custom_layout) => &custom_layout.name,
        }
    }

    pub fn preview(&self) -> Element<Message> {
        let layout = match self {
            Layout::Cosmic => LayoutPreview::new(
                Some(PanelProperties::new(Position::Top, true, 10.0)),
                Some(PanelProperties::new(Position::Bottom, true, 20.0)),
                6,
                true,
            ),
            Layout::Mac => LayoutPreview::new(
                Some(PanelProperties::new(Position::Top, true, 10.0)),
                Some(PanelProperties::new(Position::Bottom, false, 20.0)),
                6,
                true,
            ),
            Layout::Windows => LayoutPreview::new(
                None,
                Some(PanelProperties::new(Position::Bottom, true, 15.0)),
                6,
                true,
            ),
            Layout::Ubuntu => LayoutPreview::new(
                Some(PanelProperties::new(Position::Top, true, 10.0)),
                Some(PanelProperties::new(Position::Left, true, 20.0)),
                3,
                true,
            ),
            Layout::Custom(_) => LayoutPreview::new(
                Some(PanelProperties::new(Position::Top, true, 10.0)),
                None,
                0,
                true,
            ),
        };

        widget::button::custom(layout.render())
            .on_press(Message::SelectLayout(self.clone()))
            .into()
    }

    pub fn schema(&self) -> Schema {
        match self {
            Layout::Cosmic => ron::from_str::<Schema>(resources::COSMIC_LAYOUT).unwrap(),
            Layout::Mac => ron::from_str::<Schema>(resources::MAC_LAYOUT).unwrap(),
            Layout::Windows => ron::from_str::<Schema>(resources::WINDOWS_LAYOUT).unwrap(),
            Layout::Ubuntu => ron::from_str::<Schema>(resources::UBUNTU_LAYOUT).unwrap(),
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
