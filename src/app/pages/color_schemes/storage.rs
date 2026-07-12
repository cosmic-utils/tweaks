use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use anyhow::{Context, bail};
use cosmic::cosmic_theme::{Theme, ThemeBuilder};
use uuid::Uuid;

use crate::app::pages::color_schemes::models::{ColorScheme, Source};

#[derive(Debug, Clone, PartialEq)]
pub struct TempColorScheme {
    pub id: Uuid,
    pub name: String,
    pub theme_builder: ThemeBuilder,
    pub theme: Theme,
    pub author: Option<String>,
    pub link: Option<String>,
    pub downloads: Option<u64>,
    pub created: Option<i64>,
    pub updated: Option<i64>,
    pub source: Option<Source>,
    pub path: Option<PathBuf>,
}

impl From<TempColorScheme> for ColorScheme {
    fn from(value: TempColorScheme) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            theme_builder: value.theme_builder.clone(),
            author: value.author.clone(),
            link: value.link.clone(),
            downloads: value.downloads,
            created: value.created,
            updated: value.updated,
            source: value.source,
            path: value.path.clone(),
        }
    }
}

pub fn cosmic_theme_dir() -> anyhow::Result<PathBuf> {
    let dir = dirs::data_local_dir()
        .context("missing local data directory")?
        .join("themes/cosmic");

    fs::create_dir_all(&dir)?;

    Ok(dir)
}

pub fn cache_path() -> anyhow::Result<PathBuf> {
    Ok(dirs::cache_dir()
        .context("missing cache directory")?
        .join("tweaks/available_themes.json"))
}

pub fn is_cache_exist() -> bool {
    cache_path().map(|p| p.exists()).unwrap_or(false)
}

pub fn cache_themes(themes: &[ColorScheme]) -> anyhow::Result<()> {
    let path = cache_path()?;

    fs::create_dir_all(path.parent().unwrap())?;

    let file = File::create(path)?;

    serde_json::to_writer(BufWriter::new(file), &themes)?;

    Ok(())
}

pub fn validate_cache_integrity() -> bool {
    if !is_cache_exist() {
        log::warn!("failed to validate cache integrity: directory was removed");
        return false;
    }

    if let Err(err) = get_themes_from_cache() {
        log::warn!("failed to validate cache integrity: {}", err);
        std::fs::remove_file(cache_path().unwrap()).ok();
        log::warn!("cache removed, recreating...");
        return false;
    }

    true
}

pub fn get_themes_from_cache() -> anyhow::Result<Vec<ColorScheme>> {
    let file = File::open(cache_path()?)?;

    Ok(serde_json::from_reader(BufReader::new(file))?)
}

pub fn install_theme(mut theme: ColorScheme, overwrite: bool) -> anyhow::Result<ColorScheme> {
    let path = cosmic_theme_dir()?.join(&theme.name).with_extension("ron");

    if !overwrite && path.exists() {
        bail!("theme {} already exists", theme.name);
    }

    fs::write(&path, ron::ser::to_string(&theme.theme_builder)?)?;

    theme.path = Some(path);

    Ok(theme)
}

pub fn installed_system_themes() -> anyhow::Result<Vec<ColorScheme>> {
    let mut themes = Vec::new();

    let mut dirs = Vec::new();

    if let Some(local) = dirs::data_local_dir() {
        dirs.push(local.join("themes/cosmic"));
    }

    if let Ok(system) = std::env::var("XDG_DATA_DIRS") {
        dirs.extend(std::env::split_paths(&system).map(|p| p.join("themes/cosmic")));
    }

    dirs.push(PathBuf::from("/usr/local/share/themes/cosmic"));

    dirs.push(PathBuf::from("/usr/share/themes/cosmic"));

    for dir in dirs {
        let Ok(entries) = fs::read_dir(dir) else {
            continue;
        };

        for entry in entries.flatten() {
            let path = entry.path();

            let Ok(content) = fs::read_to_string(&path) else {
                continue;
            };

            let Ok(builder) = ron::from_str(&content) else {
                continue;
            };

            let Some(name) = path.file_stem().and_then(|x| x.to_str()) else {
                continue;
            };

            let mut theme = ColorScheme::new(name.to_string(), builder);

            theme.source = Some(Source::System);

            theme.path = Some(path);

            themes.push(theme);
        }
    }

    Ok(themes)
}
