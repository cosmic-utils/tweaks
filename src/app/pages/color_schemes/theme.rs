use std::{str::FromStr, sync::Arc};

use anyhow::{Context, bail};
use ashpd::desktop::file_chooser::SelectedFiles;
use cosmic::{
    cosmic_config::CosmicConfigEntry,
    cosmic_theme::{Theme, ThemeBuilder, ThemeMode},
};
use url::Url;

use crate::app::pages::color_schemes::models::{ColorScheme, Source};

pub fn import_file(f: Arc<SelectedFiles>) -> anyhow::Result<ColorScheme> {
    let Some(f) = f.uris().first() else {
        bail!("no uri")
    };

    let url = Url::from_str(f.as_str())?;

    if url.scheme() != "file" {
        bail!("scheme != file")
    }
    let Ok(path) = url.to_file_path() else {
        bail!("can't retrieve file path")
    };

    let name = path.file_stem().unwrap().to_str().unwrap().to_string();
    let content = std::fs::read_to_string(&path)?;

    let builder = ron::de::from_str(&content)?;

    let mut theme = ColorScheme::new(name, builder);

    theme.source = Some(Source::ImportedFromPath);

    let file_name = path.file_name().unwrap();

    let new_file_path = dirs::data_local_dir()
        .unwrap()
        .join("themes/cosmic")
        .join(file_name);

    std::fs::create_dir_all(new_file_path.parent().unwrap())?;
    std::fs::write(&new_file_path, &content)?;

    theme.path = Some(new_file_path);

    Ok(theme)
}

pub fn apply_theme(builder: &ThemeBuilder) -> anyhow::Result<()> {
    // Determine whether we're applying the light or dark theme.
    let is_dark = builder.palette.is_dark();

    // Ensure the current mode matches the imported theme.
    let mode_config = ThemeMode::config()?;
    let mut mode = ThemeMode::get_entry(&mode_config)
        .map(|m| m)
        .unwrap_or_default();

    if mode.is_dark != is_dark {
        mode.set_is_dark(&mode_config, is_dark)
            .context("Failed to switch theme mode")?;
    }

    // Get the appropriate configs.
    let builder_config = if is_dark {
        ThemeBuilder::dark_config()?
    } else {
        ThemeBuilder::light_config()?
    };

    let theme_config = if is_dark {
        Theme::dark_config()?
    } else {
        Theme::light_config()?
    };

    // Write the builder first.
    builder
        .write_entry(&builder_config)
        .context("Failed to write ThemeBuilder")?;

    let mut theme = builder.clone().build();
    if let Ok(current) = Theme::get_entry(&theme_config).map_err(|(_, t)| t) {
        theme.frosted_windows = current.frosted_windows;
        theme.frosted_system_interface = current.frosted_system_interface;
        theme.frosted_panel = current.frosted_panel;
        theme.frosted_applets = current.frosted_applets;
        theme.alpha_map = current.alpha_map;
    }

    // Then write the generated theme.
    theme
        .write_entry(&theme_config)
        .context("Failed to write Theme")?;

    Ok(())
}

pub fn get_current_theme() -> anyhow::Result<ThemeBuilder> {
    let mode_config = ThemeMode::config()?;

    let mode = ThemeMode::get_entry(&mode_config).unwrap_or_default();

    let config = if mode.is_dark {
        ThemeBuilder::dark_config()?
    } else {
        ThemeBuilder::light_config()?
    };

    match ThemeBuilder::get_entry(&config) {
        Ok(theme) => Ok(theme),

        Err((errors, theme)) => {
            for error in errors {
                log::error!("{error}");
            }

            Ok(theme)
        }
    }
}
