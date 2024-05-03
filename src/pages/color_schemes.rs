use std::{path::PathBuf, sync::Arc};

use self::config::ColorScheme;
use crate::{core::icons, fl};
use ashpd::desktop::file_chooser::{FileFilter, SelectedFiles};
use cosmic::{
    cosmic_config::{Config, CosmicConfigEntry},
    cosmic_theme::{Theme, ThemeBuilder, ThemeMode},
    iced::Length,
    widget, Apply, Command, Element,
};

mod config;
mod preview;

pub struct ColorSchemes {
    color_schemes: Vec<ColorScheme>,
    config_helper: Option<Config>,
    config: Option<ColorScheme>,
    selected: ColorScheme,
    theme_mode: ThemeMode,
    theme_builder_config: Option<Config>,
    theme_builder: ThemeBuilder,
}

impl Default for ColorSchemes {
    fn default() -> Self {
        let config_helper = ColorScheme::config().ok();
        let config = config_helper.as_ref().and_then(|config_helper| {
            let config = ColorScheme::get_entry(config_helper).ok()?;
            Some(config)
        });
        let theme_mode_config = ThemeMode::config().ok();
        let theme_mode = theme_mode_config
            .as_ref()
            .map(|c| match ThemeMode::get_entry(c) {
                Ok(t) => t,
                Err((errors, t)) => {
                    for e in errors {
                        eprintln!("{e}");
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

        let theme_builder = theme_builder_config.as_ref().map_or_else(
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
                        eprintln!("{e}");
                    }
                    t
                }
            },
        );
        let selected = config.clone().unwrap_or_default();
        let color_schemes = Self::fetch_color_schemes().unwrap_or_default();
        Self {
            color_schemes,
            config_helper,
            config,
            selected,
            theme_mode,
            theme_builder_config,
            theme_builder,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    StartImport,
    ImportError,
    ImportFile(Arc<SelectedFiles>),
    ImportSuccess(Box<ThemeBuilder>),
    SetColorScheme(ColorScheme),
    DeleteColorScheme(ColorScheme),
    OpenContainingFolder(ColorScheme),
    ReloadColorSchemes,
}

impl ColorSchemes {
    pub fn view<'a>(&self) -> Element<'a, Message> {
        let spacing = cosmic::theme::active().cosmic().spacing;
        widget::column::with_children(vec![
            widget::row::with_children(vec![
                widget::text::title3(fl!("color-schemes")).into(),
                widget::horizontal_space(Length::Fill).into(),
                icons::get_icon("step-in-symbolic", 16)
                    .apply(widget::button)
                    .padding(spacing.space_xxs)
                    .on_press(Message::StartImport)
                    .style(cosmic::theme::Button::Standard)
                    .into(),
            ])
            .into(),
            widget::scrollable(widget::settings::view_section(fl!("installed")).add({
                let themes: Vec<Element<Message>> = self
                    .color_schemes
                    .iter()
                    .map(|color_scheme| preview::view(color_scheme, &self.selected))
                    .collect();

                widget::flex_row(themes)
                    .row_spacing(spacing.space_xs)
                    .column_spacing(spacing.space_xs)
                    .apply(widget::container)
                    .padding([0, spacing.space_xxs])
            }))
            .into(),
        ])
        .spacing(spacing.space_xxs)
        .into()
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        let mut commands = vec![];
        match message {
            Message::StartImport => commands.push(Command::perform(
                async {
                    SelectedFiles::open_file()
                        .modal(true)
                        .filter(FileFilter::glob(FileFilter::new("ron"), "*.ron"))
                        .send()
                        .await?
                        .response()
                },
                |res| {
                    if let Ok(f) = res {
                        Message::ImportFile(Arc::new(f))
                    } else {
                        // TODO Error toast?
                        eprintln!("failed to select a file for importing a custom theme.");
                        Message::ImportError
                    }
                },
            )),
            Message::ImportError => eprintln!("failed to import a custom theme."),
            Message::ImportFile(f) => {
                let Some(f) = f.uris().first() else {
                    return Command::none();
                };
                if f.scheme() != "file" {
                    return Command::none();
                }
                let Ok(path) = f.to_file_path() else {
                    return Command::none();
                };
                let inner_path = path.clone();
                commands.push(Command::perform(
                    async move { tokio::fs::read_to_string(path).await },
                    move |res| {
                        if let Some(b) = res.ok().and_then(|s| {
                            if let Some(dir) = dirs::data_local_dir() {
                                if inner_path.is_file() {
                                    let file = inner_path
                                        .file_name()
                                        .unwrap()
                                        .to_str()
                                        .unwrap()
                                        .to_string();
                                    let dir = dir.join("themes/cosmic").join(file);
                                    if !dir.exists() {
                                        if let Err(e) = std::fs::write(dir, &s) {
                                            eprintln!(
                                                "failed to write the file to the themes directory: {e}"
                                            );
                                        }
                                    }
                                }
                            }
                            ron::de::from_str(&s).ok()
                        }) {
                            Message::ImportSuccess(Box::new(b))
                        } else {
                            eprintln!("failed to import a file for a custom theme.");
                            Message::ImportError
                        }
                    },
                ))
            }
            Message::ImportSuccess(builder) => {
                self.theme_builder = *builder;

                if let Some(config) = self.theme_builder_config.as_ref() {
                    _ = self.theme_builder.write_entry(config);
                };

                let config = if self.theme_mode.is_dark {
                    Theme::dark_config()
                } else {
                    Theme::light_config()
                };
                let new_theme = self.theme_builder.clone().build();
                if let Ok(config) = config {
                    _ = new_theme.write_entry(&config);
                } else {
                    eprintln!("Failed to get the theme config.");
                }
                commands.push(self.update(Message::ReloadColorSchemes));
            }
            Message::SetColorScheme(color_scheme) => {
                self.selected = color_scheme.clone();
                let Some(config_helper) = &self.config_helper else {
                    return Command::none();
                };
                let Some(config) = &mut self.config else {
                    return Command::none();
                };
                if let Err(e) = config.set_name(&config_helper, self.selected.name.clone()) {
                    eprintln!("There was an error selecting the color scheme: {e}");
                }
                if let Err(e) = config.set_path(&config_helper, self.selected.path.clone()) {
                    eprintln!("There was an error selecting the color scheme: {e}");
                }
                if let Ok(theme) = &color_scheme.theme() {
                    commands.push(self.update(Message::ImportSuccess(Box::new(theme.clone()))))
                }
            }
            Message::DeleteColorScheme(color_scheme) => {
                if self.selected.name == color_scheme.name {
                    if let Some(color_scheme) = self.color_schemes.first() {
                        commands.push(self.update(Message::SetColorScheme(color_scheme.clone())));
                        commands.push(self.update(Message::ReloadColorSchemes));
                    }
                }
                std::fs::remove_file(&color_scheme.path).unwrap_or_else(|e| {
                    eprintln!("There was an error deleting the color scheme: {e}")
                });
            }
            Message::OpenContainingFolder(color_scheme) => {
                if let Some(path) = color_scheme.path.parent() {
                    if let Err(e) = open::that_detached(path) {
                        eprintln!("There was an error opening that color scheme: {e}")
                    }
                }
            }
            Message::ReloadColorSchemes => {
                self.color_schemes = Self::fetch_color_schemes().unwrap_or_default();
            }
        }
        Command::batch(commands)
    }

    pub fn fetch_color_schemes() -> anyhow::Result<Vec<ColorScheme>> {
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
                let color_scheme = ColorScheme { name, path, theme };
                color_schemes.push(color_scheme);
            }
        }

        Ok(color_schemes)
    }
}
