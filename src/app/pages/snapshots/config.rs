use std::{fmt::Display, path::PathBuf};

use crate::{app::App, fl};
use chrono::{NaiveDateTime, Utc};
use cosmic::Application;
use cosmic_ext_config_templates::{panel::PanelSchema, Schema};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Clone, Default, Deserialize)]
pub struct Snapshot {
    pub id: Uuid,
    pub name: String,
    pub kind: SnapshotKind,
    pub created: NaiveDateTime,
    pub schema: Option<Schema>,
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
    pub fn new(name: impl ToString, kind: SnapshotKind) -> Self {
        let id = Uuid::new_v4();
        let created = Utc::now().naive_local();

        Self {
            id,
            name: name.to_string(),
            kind,
            created,
            schema: PanelSchema::generate()
                .ok()
                .map(|panel_schema| Schema::Panel(panel_schema)),
        }
    }

    pub fn created(&self) -> String {
        self.created.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    pub fn schema(&self) -> Schema {
        Schema::from_file(&self.path()).unwrap()
    }

    pub fn path(&self) -> PathBuf {
        dirs::data_local_dir()
            .unwrap()
            .join(App::APP_ID)
            .join("snapshots")
            .join(&self.id.to_string())
            .with_extension("ron")
    }
}
