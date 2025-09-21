use std::collections::HashMap;

use cosmic::cosmic_config;
use cosmic_config::{CosmicConfigEntry, cosmic_config_derive::CosmicConfigEntry};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, CosmicConfigEntry)]
#[version = 1]
#[serde(deny_unknown_fields)]
pub struct CosmicPanelButtonConfig {
    pub configs: HashMap<String, IndividualConfig>,
}

impl Default for CosmicPanelButtonConfig {
    fn default() -> Self {
        Self {
            configs: HashMap::from([
                (
                    "Panel".to_string(),
                    IndividualConfig {
                        force_presentation: None,
                    },
                ),
                (
                    "Dock".to_string(),
                    IndividualConfig {
                        force_presentation: Some(Override::Icon),
                    },
                ),
            ]),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Default, Clone)]
pub struct IndividualConfig {
    pub force_presentation: Option<Override>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Override {
    Icon,
    Text,
}
