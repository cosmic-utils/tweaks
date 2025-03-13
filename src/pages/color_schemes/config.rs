use std::path::PathBuf;

use cosmic::{
    cosmic_config::{self, Config},
    cosmic_theme::ThemeBuilder,
    iced::{Alignment, Length},
    widget::{self, JustifyContent},
    Element,
};
use cosmic_config::cosmic_config_derive::CosmicConfigEntry;
use cosmic_config::CosmicConfigEntry;
use serde::{Deserialize, Serialize};

use super::{preview, providers::cosmic_themes::CosmicTheme};

pub const COLOR_SCHEME_CONFIG_ID: &str = "dev.edfloreshz.CosmicTweaks.ColorScheme";

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

    /// Get the config for the theme
    pub fn config() -> Result<Config, cosmic_config::Error> {
        Config::new(COLOR_SCHEME_CONFIG_ID, Self::VERSION)
    }

    pub fn theme(&self) -> anyhow::Result<ThemeBuilder> {
        let Some(path) = self.path.as_ref() else {
            anyhow::bail!("No path for the theme")
        };

        let file = std::fs::read_to_string(path)?;
        let theme: ThemeBuilder = ron::from_str(&file)?;
        Ok(theme)
    }

    pub fn grid_metrics(spacing: &cosmic::cosmic_theme::Spacing, width: usize) -> GridMetrics {
        GridMetrics::new(width, 240 + 2 * spacing.space_s as usize, spacing.space_xxs)
    }

    pub fn installed_grid<'a>(
        color_schemes: &'a [Self],
        selected: &Self,
        spacing: cosmic::cosmic_theme::Spacing,
        width: usize,
    ) -> Element<'a, super::Message> {
        let GridMetrics {
            cols,
            item_width,
            column_spacing,
        } = Self::grid_metrics(&spacing, width);

        let mut grid = widget::grid();
        let mut col = 0;
        for color_scheme in color_schemes.iter() {
            if col >= cols {
                grid = grid.insert_row();
                col = 0;
            }
            grid = grid.push(preview::installed(
                color_scheme,
                &selected,
                &spacing,
                item_width,
            ));
            col += 1;
        }
        grid.column_spacing(column_spacing)
            .row_spacing(column_spacing)
            .row_alignment(Alignment::Center)
            .column_alignment(Alignment::Center)
            .justify_content(JustifyContent::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn available_grid<'a>(
        color_schemes: &'a [Self],
        spacing: cosmic::cosmic_theme::Spacing,
        width: usize,
    ) -> Element<'a, super::Message> {
        let GridMetrics {
            cols,
            item_width,
            column_spacing,
        } = Self::grid_metrics(&spacing, width);

        let mut grid = widget::grid();
        let mut col = 0;
        for color_scheme in color_schemes.iter() {
            if col >= cols {
                grid = grid.insert_row();
                col = 0;
            }

            grid = grid.push(preview::available(color_scheme, &spacing, item_width));
            col += 1;
        }

        grid.column_spacing(column_spacing)
            .row_spacing(column_spacing)
            .row_alignment(Alignment::Center)
            .column_alignment(Alignment::Center)
            .justify_content(JustifyContent::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl From<CosmicTheme> for ColorScheme {
    fn from(theme: CosmicTheme) -> Self {
        Self {
            name: theme.name,
            path: None,
            link: Some(theme.link),
            author: Some(theme.author),
            theme: ron::from_str(&theme.ron).unwrap(),
        }
    }
}

pub struct GridMetrics {
    pub cols: usize,
    pub item_width: usize,
    pub column_spacing: u16,
}

impl GridMetrics {
    pub fn new(width: usize, min_width: usize, column_spacing: u16) -> Self {
        let width_m1 = width.checked_sub(min_width).unwrap_or(0);
        let cols_m1 = width_m1 / (min_width + column_spacing as usize);
        let cols = cols_m1 + 1;
        let item_width = width
            .checked_sub(cols_m1 * column_spacing as usize)
            .unwrap_or(0)
            .checked_div(cols)
            .unwrap_or(0);
        Self {
            cols,
            item_width,
            column_spacing,
        }
    }
}
