use std::path::PathBuf;

use cosmic::{
    cosmic_config::{self, Config},
    cosmic_theme::{ThemeBuilder, ThemeMode},
};
use cosmic_config::CosmicConfigEntry;
use cosmic_config::cosmic_config_derive::CosmicConfigEntry;
use serde::{Deserialize, Serialize};

use crate::Error;

const CONFIG_ID: &str = "dev.edfloreshz.CosmicTweaks.ColorScheme";

#[derive(Debug, Serialize, Clone, Default, Deserialize, PartialEq, CosmicConfigEntry)]
#[version = 1]
pub struct ColorScheme {
    pub name: String,
    pub path: Option<PathBuf>,
    pub link: Option<String>,
    pub author: Option<String>,
    pub theme: ThemeBuilder,
}

impl ColorScheme {
    #[allow(dead_code)]
    pub const fn version() -> u64 {
        Self::VERSION
    }

    pub fn config() -> Config {
        match Config::new(CONFIG_ID, Self::VERSION) {
            Ok(config) => config,
            Err(err) => panic!("Failed to load config: {}", err),
        }
    }

    pub fn read_theme(&self) -> Result<ThemeBuilder, Error> {
        let Some(path) = self.path.as_ref() else {
            return Err(Error::ThemePathNotFound);
        };

        let file = std::fs::read_to_string(path)?;
        let theme: ThemeBuilder = ron::from_str(&file)?;
        Ok(theme)
    }

    pub fn current_theme() -> ThemeBuilder {
        let theme_mode_config = ThemeMode::config().ok();
        let theme_mode = theme_mode_config
            .as_ref()
            .map(|c| match ThemeMode::get_entry(c) {
                Ok(t) => t,
                Err((errors, t)) => {
                    for e in errors {
                        log::error!("{e}");
                    }
                    t
                }
            })
            .unwrap_or_default();
        let theme_builder_config = if theme_mode.is_dark {
            ThemeBuilder::dark_config()
        } else {
            ThemeBuilder::light_config()
        }
        .ok();

        theme_builder_config.as_ref().map_or_else(
            || {
                if theme_mode.is_dark {
                    ThemeBuilder::dark()
                } else {
                    ThemeBuilder::light()
                }
            },
            |c| match ThemeBuilder::get_entry(c) {
                Ok(t) => t,
                Err((errors, t)) => {
                    for e in errors {
                        log::error!("{e}");
                    }
                    t
                }
            },
        )
    }

    pub fn installed() -> Result<Vec<Self>, Error> {
        let mut color_schemes = vec![];

        let xdg_data_home = std::env::var("XDG_DATA_HOME")
            .ok()
            .and_then(|value| {
                if value.is_empty() {
                    None
                } else {
                    Some(PathBuf::from(value))
                }
            })
            .or_else(dirs::data_local_dir)
            .map(|dir| dir.join("themes/cosmic"));

        if let Some(ref xdg_data_home) = xdg_data_home
            && !xdg_data_home.exists()
            && let Err(e) = std::fs::create_dir_all(xdg_data_home)
        {
            log::error!("failed to create the themes directory: {e}")
        };

        let xdg_data_dirs = std::env::var("XDG_DATA_DIRS").ok();

        let xdg_data_dirs = xdg_data_dirs
            .as_deref()
            .or(Some("/usr/local/share/:/usr/share/"))
            .into_iter()
            .flat_map(|arg| std::env::split_paths(arg).map(|dir| dir.join("themes/cosmic")));

        for themes_directory in xdg_data_dirs.chain(xdg_data_home) {
            let Ok(read_dir) = std::fs::read_dir(&themes_directory) else {
                continue;
            };

            for entry in read_dir.filter_map(Result::ok) {
                let path = entry.path();
                let color_scheme = std::fs::read_to_string(&path)?;
                let theme: ThemeBuilder = ron::from_str(&color_scheme)?;
                let name = path
                    .file_stem()
                    .and_then(|name| name.to_str())
                    .map(|name| name.to_string())
                    .unwrap_or_default();
                let color_scheme = ColorScheme {
                    name,
                    path: Some(path),
                    link: None,
                    author: None,
                    theme,
                };
                color_schemes.push(color_scheme);
            }
        }

        Ok(color_schemes)
    }
}
