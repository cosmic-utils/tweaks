use std::{path::PathBuf, sync::Arc};

use self::config::ColorScheme;
use crate::{core::icons, fl};
use ashpd::desktop::file_chooser::{FileFilter, SelectedFiles};
use cosmic::{
    cosmic_config::{Config, CosmicConfigEntry},
    cosmic_theme::{Theme, ThemeBuilder, ThemeMode},
    iced::Length,
    widget::{self, tooltip},
    Apply, Command, Element,
};

pub mod config;
pub mod preview;
pub mod providers;

pub struct ColorSchemes {
    selected: ColorScheme,
    installed: Vec<ColorScheme>,
    config_helper: Option<Config>,
    config: Option<ColorScheme>,
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
                        log::error!("{e}");
                    }
                    t
                }
            },
        );
        let selected = config.clone().unwrap_or_default();
        let installed = Self::fetch_color_schemes().unwrap_or_default();
        Self {
            selected,
            installed,
            config_helper,
            config,
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
    SaveCurrentColorScheme(Option<String>),
    SetColorScheme(ColorScheme),
    DeleteColorScheme(ColorScheme),
    InstallColorScheme(ColorScheme),
    OpenContainingFolder(ColorScheme),
    OpenLink(Option<String>),
    ReloadColorSchemes,
    OpenAvailableThemes,
}

#[derive(Debug, Clone)]
pub enum ColorSchemeProvider {
    CosmicThemes,
}

impl ColorSchemes {
    pub fn view<'a>(&self) -> Element<'a, Message> {
        let spacing = cosmic::theme::active().cosmic().spacing;

        widget::column::with_children(vec![
            widget::row::with_children(vec![
                widget::text::title3(fl!("color-schemes")).into(),
                widget::horizontal_space(Length::Fill).into(),
                widget::tooltip::tooltip(
                    icons::get_handle("arrow-into-box-symbolic", 16)
                        .apply(widget::button::icon)
                        .padding(spacing.space_xxs)
                        .on_press(Message::SaveCurrentColorScheme(None))
                        .style(cosmic::theme::Button::Standard),
                    fl!("save-current-color-scheme"),
                    tooltip::Position::Bottom,
                )
                .into(),
                widget::tooltip::tooltip(
                    icons::get_handle("document-save-symbolic", 16)
                        .apply(widget::button::icon)
                        .padding(spacing.space_xxs)
                        .on_press(Message::StartImport)
                        .style(cosmic::theme::Button::Standard),
                    fl!("import-color-scheme"),
                    tooltip::Position::Bottom,
                )
                .into(),
                widget::tooltip::tooltip(
                    icons::get_handle("search-global-symbolic", 16)
                        .apply(widget::button::icon)
                        .padding(spacing.space_xxs)
                        .on_press(Message::OpenAvailableThemes)
                        .style(cosmic::theme::Button::Standard),
                    fl!("find-color-schemes"),
                    tooltip::Position::Bottom,
                )
                .into(),
            ])
            .spacing(spacing.space_xxs)
            .into(),
            widget::settings::section()
                .title(fl!("installed"))
                .add({
                    let themes: Vec<Element<Message>> = self
                        .installed
                        .iter()
                        .map(|color_scheme| preview::installed(color_scheme, &self.selected))
                        .collect();

                    widget::flex_row(themes)
                        .row_spacing(spacing.space_xs)
                        .column_spacing(spacing.space_xs)
                        .apply(widget::container)
                        .padding([0, spacing.space_xxs])
                })
                .into(),
        ])
        .spacing(spacing.space_xxs)
        .apply(widget::scrollable)
        .into()
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        let mut commands = vec![];
        match message {
            Message::OpenAvailableThemes => {
                commands.push(self.update(Message::OpenAvailableThemes))
            }
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
                        log::error!("failed to select a file for importing a custom theme.");
                        Message::ImportError
                    }
                },
            )),
            Message::ImportError => log::error!("failed to import a custom theme."),
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

                let file = path.file_name().unwrap().to_str().unwrap().to_string();

                let new_file = dirs::data_local_dir()
                    .map(|dir| dir.join("themes/cosmic").join(file))
                    .unwrap_or_default();

                let color_scheme = ColorScheme {
                    name: new_file.file_stem().unwrap().to_str().unwrap().to_string(),
                    path: Some(new_file.clone()),
                    link: None,
                    author: None,
                    theme: Default::default(),
                };

                commands.push(self.update(Message::SetColorScheme(color_scheme.clone())));

                let file_path = path.clone();
                commands.push(Command::perform(
                    async move { tokio::fs::read_to_string(path).await },
                    move |res| {
                        if let Some(b) = res.ok().and_then(|theme| {
                            if file_path.is_file() && !new_file.exists() {
                                if let Err(e) = std::fs::write(new_file, &theme) {
                                    log::error!(
                                        "failed to write the file to the themes directory: {e}"
                                    );
                                }
                            }
                            ron::de::from_str(&theme).ok()
                        }) {
                            Message::ImportSuccess(Box::new(b))
                        } else {
                            log::error!("failed to import a file for a custom theme.");
                            Message::ImportError
                        }
                    },
                ))
            }
            Message::ImportSuccess(builder) => {
                self.theme_builder = *builder;

                let Some(config) = self.theme_builder_config.as_ref() else {
                    log::error!("Failed to get the theme config.");
                    return Command::none();
                };

                if let Err(e) = self.theme_builder.write_entry(config) {
                    log::error!("Failed to write the theme config: {e}");
                }

                let config = if self.theme_mode.is_dark {
                    Theme::dark_config()
                } else {
                    Theme::light_config()
                };
                let new_theme = self.theme_builder.clone().build();

                let Some(config) = config.ok() else {
                    log::error!("Failed to get the theme config.");
                    return Command::none();
                };

                if let Err(e) = new_theme.write_entry(&config) {
                    log::error!("Failed to write the theme config: {e}");
                }
                commands.push(self.update(Message::ReloadColorSchemes));
            }
            Message::SetColorScheme(color_scheme) => {
                self.selected = color_scheme.clone();
                let Some(config_helper) = &self.config_helper else {
                    log::error!("Failed to get the config helper.");
                    return Command::none();
                };
                let Some(config) = &mut self.config else {
                    log::error!("Failed to get the config.");
                    return Command::none();
                };
                if let Err(e) = config.set_name(config_helper, self.selected.name.clone()) {
                    log::error!("There was an error selecting the color scheme: {e}");
                }
                if let Err(e) = config.set_path(config_helper, self.selected.path.clone()) {
                    log::error!("There was an error selecting the color scheme: {e}");
                }

                if color_scheme.theme != ThemeBuilder::default() {
                    log::info!("Theme is not default, setting the theme...");
                    if let Ok(theme) = &color_scheme.theme() {
                        log::info!("Color scheme has a theme, setting the theme...");
                        commands.push(self.update(Message::ImportSuccess(Box::new(theme.clone()))))
                    }
                }
            }
            Message::DeleteColorScheme(color_scheme) => {
                if self.selected.name == color_scheme.name {
                    if let Some(color_scheme) = self.installed.first() {
                        commands.push(self.update(Message::SetColorScheme(color_scheme.clone())));
                        commands.push(self.update(Message::ReloadColorSchemes));
                    }
                }
                let Some(path) = color_scheme.path else {
                    return Command::none();
                };
                std::fs::remove_file(&path).unwrap_or_else(|e| {
                    log::error!("There was an error deleting the color scheme: {e}")
                });
                commands.push(self.update(Message::ReloadColorSchemes));
            }
            Message::InstallColorScheme(color_scheme) => {
                let new_file = dirs::data_local_dir()
                    .map(|dir| {
                        dir.join("themes/cosmic")
                            .join(&color_scheme.name)
                            .with_extension("ron")
                    })
                    .unwrap_or_default();

                if let Err(e) =
                    std::fs::write(&new_file, ron::ser::to_string(&color_scheme.theme).unwrap())
                {
                    log::error!("There was an error installing the color scheme: {e}");
                }
                commands.push(self.update(Message::ReloadColorSchemes));
            }
            Message::OpenLink(link) => {
                if let Some(link) = link {
                    open::that_detached(link).unwrap_or_else(|e| {
                        log::error!("There was an error opening the link: {e}")
                    });
                }
            }
            Message::OpenContainingFolder(color_scheme) => {
                let Some(path) = color_scheme.path else {
                    return Command::none();
                };
                if let Some(path) = path.parent() {
                    if let Err(e) = open::that_detached(path) {
                        log::error!("There was an error opening that color scheme: {e}")
                    }
                }
            }
            Message::ReloadColorSchemes => {
                self.installed = Self::fetch_color_schemes().unwrap_or_default();
            }
            Message::SaveCurrentColorScheme(name) => {
                if let Some(name) = name {
                    let path = dirs::data_local_dir()
                        .map(|dir| dir.join("themes/cosmic").join(&name).with_extension("ron"))
                        .unwrap_or_default();

                    let color_scheme = ColorScheme {
                        name,
                        path: Some(path.clone()),
                        link: None,
                        author: None,
                        theme: self.theme_builder.clone(),
                    };

                    if path.exists() {
                        log::error!("The color scheme already exists.");
                        return Command::none();
                    }

                    let Ok(theme_builder) = ron::to_string(&self.theme_builder) else {
                        log::error!("failed to serialize the theme builder");
                        return Command::none();
                    };

                    if let Err(e) = std::fs::write(path, theme_builder) {
                        log::error!("failed to write the file to the themes directory: {e}");
                    }

                    commands.push(self.update(Message::SetColorScheme(color_scheme)));
                    commands.push(self.update(Message::ReloadColorSchemes))
                } else {
                    commands.push(self.update(Message::SaveCurrentColorScheme(None)))
                }
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

        if let Some(ref xdg_data_home) = xdg_data_home {
            if !xdg_data_home.exists() {
                if let Err(e) = std::fs::create_dir_all(xdg_data_home) {
                    log::error!("failed to create the themes directory: {e}")
                };
            }
        }

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
