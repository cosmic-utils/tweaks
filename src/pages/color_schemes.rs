use std::{path::PathBuf, sync::OnceLock};

use self::config::ColorScheme;
use crate::fl;
use cosmic::{
    cosmic_config::{Config, CosmicConfigEntry},
    cosmic_theme::ThemeBuilder,
    iced::{Alignment, Length},
    widget, Apply, Command, Element,
};

mod config;
mod preview;

static COLOR_SCHEMES: OnceLock<Vec<ColorScheme>> = OnceLock::new();

pub struct ColorSchemes {
    config_helper: Option<Config>,
    config: Option<ColorScheme>,
    selected: ColorScheme,
}

impl Default for ColorSchemes {
    fn default() -> Self {
        let _ = Self::fetch_color_schemes();
        let config_helper = ColorScheme::config().ok();
        let config = config_helper.as_ref().and_then(|config_helper| {
            let config = ColorScheme::get_entry(config_helper).ok()?;
            Some(config)
        });
        let selected = config.clone().unwrap_or_default();
        Self {
            config_helper,
            config,
            selected,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ImportColorScheme(String),
    SetColorScheme(ColorScheme),
    DeleteColorScheme(ColorScheme),
    OpenContainingFolder(ColorScheme),
}

impl ColorSchemes {
    pub fn view<'a>(&self) -> Element<'a, Message> {
        widget::scrollable(widget::settings::view_section(fl!("color-schemes")).add({
            let spacing = cosmic::theme::active().cosmic().spacing;
            let Ok(()) = Self::fetch_color_schemes() else {
                return widget::row::with_children(vec![
                    widget::text(fl!("color-schemes-error")).into()
                ])
                .align_items(Alignment::Center)
                .width(Length::Fill)
                .height(Length::Fill)
                .into();
            };

            let themes: Vec<Element<Message>> = COLOR_SCHEMES
                .get()
                .unwrap_or(&vec![])
                .iter()
                .map(|color_scheme| preview::view(&color_scheme, &self.selected))
                .collect();

            widget::flex_row(themes)
                .row_spacing(spacing.space_xs)
                .column_spacing(spacing.space_xs)
                .apply(widget::container)
                .padding([0, spacing.space_xxs])
        }))
        .into()
    }

    pub fn update(&mut self, message: Message) -> Command<crate::app::Message> {
        match message {
            Message::ImportColorScheme(name) => println!("{}", name),
            Message::SetColorScheme(color_scheme) => println!("{}", color_scheme.name),
            Message::DeleteColorScheme(color_scheme) => println!("{}", color_scheme.name),
            Message::OpenContainingFolder(color_scheme) => println!("{}", color_scheme.name),
        }
        Command::none()
    }

    pub fn fetch_color_schemes() -> anyhow::Result<()> {
        if COLOR_SCHEMES.get().is_none() {
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
                    let color_scheme = ColorScheme { name, theme };
                    color_schemes.push(color_scheme);
                }
            }
            COLOR_SCHEMES.get_or_init(|| color_schemes);
        }

        Ok(())
    }
}
