use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use crate::{app::App, fl};
use chrono::{NaiveDateTime, Utc};
use cosmic::{
    cosmic_config::{self, cosmic_config_derive::CosmicConfigEntry, Config, CosmicConfigEntry},
    Application,
};
use cosmic_ext_config_templates::Schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Default, Clone, Deserialize, PartialEq, CosmicConfigEntry)]
#[version = 1]
pub struct SnapshotsConfig {
    pub snapshots: Vec<Snapshot>,
}

impl SnapshotsConfig {
    pub fn helper() -> Config {
        match Config::new(App::APP_ID, Self::VERSION) {
            Ok(config) => config,
            Err(err) => panic!("error loading config: {}", err),
        }
    }

    pub fn config() -> SnapshotsConfig {
        SnapshotsConfig::get_entry(&Self::helper()).unwrap_or_else(|(errs, config)| {
            log::info!("errors loading config: {:?}", errs);
            config
        })
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

impl Display for SnapshotKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::System => write!(f, "{}", fl!("system")),
            Self::User => write!(f, "{}", fl!("user")),
        }
    }
}

impl Snapshot {
    pub fn new(name: &str, path: &Path, kind: SnapshotKind) -> Self {
        let created = Utc::now().naive_local();
        let path = path.join(name).with_extension("ron");

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
