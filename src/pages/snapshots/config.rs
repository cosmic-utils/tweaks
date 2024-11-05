use std::path::PathBuf;

use crate::app::TweakTool;
use chrono::{NaiveDateTime, Utc};
use cosmic::{
    cosmic_config::{self, cosmic_config_derive::CosmicConfigEntry, Config, CosmicConfigEntry},
    Application,
};
use cosmic_ext_config_templates::Schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize, PartialEq, CosmicConfigEntry)]
#[version = 1]
pub struct SnapshotsConfig {
    pub snapshots: Vec<Snapshot>,
}

impl Default for SnapshotsConfig {
    fn default() -> Self {
        Self { snapshots: vec![] }
    }
}

impl SnapshotsConfig {
    pub fn helper() -> Option<Config> {
        Config::new(TweakTool::APP_ID, Self::VERSION).ok()
    }

    pub fn config() -> SnapshotsConfig {
        match Self::helper() {
            Some(config_handler) => {
                SnapshotsConfig::get_entry(&config_handler).unwrap_or_else(|(errs, config)| {
                    log::info!("errors loading config: {:?}", errs);
                    config
                })
            }
            None => SnapshotsConfig::default(),
        }
    }
}

#[derive(
    Debug, Serialize, Clone, Default, Deserialize, PartialEq, Eq, PartialOrd, Ord, CosmicConfigEntry,
)]
pub struct Snapshot {
    path: PathBuf,
    created: NaiveDateTime,
}

impl Snapshot {
    pub fn new(path: &PathBuf) -> Self {
        let created = Utc::now().naive_local();
        let mut path = path.join(&created.format("%Y-%m-%d %H:%M:%S").to_string());
        path.set_extension("ron");

        Self { path, created }
    }

    pub fn name(&self) -> String {
        self.created().format("%Y-%m-%d %H:%M:%S").to_string()
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn created(&self) -> &NaiveDateTime {
        &self.created
    }

    pub fn schema(&self) -> Schema {
        Schema::from_file(self.path()).unwrap()
    }
}
