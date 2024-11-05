use std::path::PathBuf;

use crate::{app::TweakTool, fl};
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
    pub name: String,
    pub kind: SnapshotKind,
    pub path: PathBuf,
    pub created: NaiveDateTime,
}

#[derive(Debug, Serialize, Clone, Default, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum SnapshotKind {
    #[default]
    System,
    User,
}

impl ToString for SnapshotKind {
    fn to_string(&self) -> String {
        match self {
            Self::System => fl!("system"),
            Self::User => fl!("user"),
        }
    }
}

impl Snapshot {
    pub fn new(name: &str, path: &PathBuf, kind: SnapshotKind) -> Self {
        let created = Utc::now().naive_local();
        let path = path.join(&name).with_extension("ron");

        Self {
            name: name.to_string(),
            kind,
            path,
            created,
        }
    }

    pub fn kind(&self) -> String {
        self.kind.to_string()
    }

    pub fn created(&self) -> String {
        self.created.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    pub fn schema(&self) -> Schema {
        Schema::from_file(&self.path).unwrap()
    }
}
