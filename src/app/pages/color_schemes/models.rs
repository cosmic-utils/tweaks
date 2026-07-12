use std::fmt::Display;
use std::path::PathBuf;

use cosmic::cosmic_theme::ThemeBuilder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::localize::LANGUAGE_SORTER;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum SortBy {
    Az,
    MostDownloaded,
    #[default]
    LastModified,
    Author,
}

impl Display for SortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortBy::Az => write!(f, "{}", fl!("a-z")),
            SortBy::MostDownloaded => write!(f, "{}", fl!("most-downloaded")),
            SortBy::LastModified => write!(f, "{}", fl!("last-modified")),
            SortBy::Author => write!(f, "{}", fl!("author")),
        }
    }
}

impl SortBy {
    pub fn compare(&self, a: &ColorScheme, b: &ColorScheme) -> std::cmp::Ordering {
        match self {
            SortBy::Az => LANGUAGE_SORTER.compare(&a.name, &b.name),

            SortBy::MostDownloaded => match (a.downloads, b.downloads) {
                (None, None) => std::cmp::Ordering::Equal,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (Some(_), None) => std::cmp::Ordering::Less,
                (Some(a), Some(b)) => b.cmp(&a),
            },

            SortBy::LastModified => match (a.updated, b.updated) {
                (None, None) => std::cmp::Ordering::Equal,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (Some(_), None) => std::cmp::Ordering::Less,
                (Some(a), Some(b)) => b.cmp(&a),
            },

            SortBy::Author => match (&a.author, &b.author) {
                (None, None) => std::cmp::Ordering::Equal,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (Some(_), None) => std::cmp::Ordering::Less,
                (Some(a), Some(b)) => LANGUAGE_SORTER.compare(a, b),
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Status {
    Idle,
    Loading,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Installed,
    Available,
}

#[derive(Debug, Clone)]
pub enum ColorSchemeKey {
    Installed(String),
    Available(usize),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Source {
    CosmicThemesOrg,
    ImportedFromPath,
    Saved,
    System,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorScheme {
    pub id: Uuid,
    pub name: String,
    pub theme_builder: ThemeBuilder,
    pub author: Option<String>,
    pub link: Option<String>,
    pub downloads: Option<u64>,
    pub created: Option<i64>,
    pub updated: Option<i64>,
    pub source: Option<Source>,
    pub path: Option<PathBuf>,
}

impl ColorScheme {
    pub fn new(name: String, theme: ThemeBuilder) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            theme_builder: theme,
            author: None,
            link: None,
            downloads: None,
            created: None,
            updated: None,
            source: None,
            path: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageErrorKind {
    Fetching,
    Other,
}
