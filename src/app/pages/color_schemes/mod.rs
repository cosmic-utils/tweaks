use std::sync::Arc;

use self::config::ColorScheme;
use crate::app::core::grid::GridMetrics;
use crate::fl;
use ashpd::desktop::file_chooser::{FileFilter, SelectedFiles};
use cosmic::{
    Element, Task,
    cosmic_config::CosmicConfigEntry,
    cosmic_theme::{Theme, ThemeBuilder, ThemeMode},
    iced::Length,
    widget::{
        self,
        segmented_button::{self, SingleSelect},
    },
};
use cosmic_theme::CosmicTheme;

pub mod config;
pub mod cosmic_theme;
pub mod preview;

pub struct ColorSchemes {
    installed: Vec<ColorScheme>,
    available: Vec<ColorScheme>,
    color_scheme: ColorScheme,
    pub theme_builder: ThemeBuilder,
    pub model: segmented_button::Model<SingleSelect>,
    pub status: Status,
    pub limit: usize,
    offset: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Status {
    Idle,
    Loading,
    LoadingMore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Installed,
    Available,
}

impl Default for ColorSchemes {
    fn default() -> Self {
        Self {
            installed: ColorScheme::installed().unwrap_or_default(),
            available: vec![],
            color_scheme: match ColorScheme::get_entry(&ColorScheme::config()) {
                Ok(config) => config,
                Err((errors, default)) => {
                    log::error!("Failed to load color scheme config: {errors:#?}");
                    default
                }
            },
            model: segmented_button::Model::builder()
                .insert(|b| b.text("Installed").data(Tab::Installed).activate())
                .insert(|b| b.text("Available").data(Tab::Available))
                .build(),
            theme_builder: ColorScheme::current_theme(),
            status: Status::Idle,
            limit: 15,
            offset: 0,
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
    FetchAvailableColorSchemes(ColorSchemeProvider, usize),
    SetAvailableColorSchemes(Vec<ColorScheme>),
    OpenContainingFolder(ColorScheme),
    OpenLink(Option<String>),
    ReloadColorSchemes,
    TabSelected(segmented_button::Entity),
}

#[derive(Debug, Clone)]
pub enum ColorSchemeProvider {
    CosmicThemes,
}

impl ColorSchemes {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        let mut tasks = vec![];
        match message {
            Message::TabSelected(entity) => {
                self.model.activate(entity);
            }
            Message::StartImport => tasks.push(Task::perform(
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
                    return Task::none();
                };
                if f.scheme() != "file" {
                    return Task::none();
                }
                let Ok(path) = f.to_file_path() else {
                    return Task::none();
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

                tasks.push(self.update(Message::SetColorScheme(color_scheme.clone())));

                let file_path = path.clone();
                tasks.push(Task::perform(
                    async move { (tokio::fs::read_to_string(path).await, file_path) },
                    move |(res, path)| {
                        if let Some(b) = res.ok().and_then(|theme| {
                            if path.is_file()
                                && !path.exists()
                                && let Err(e) = std::fs::write(path, &theme)
                            {
                                log::error!(
                                    "failed to write the file to the themes directory: {e}"
                                );
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

                let config = if theme_mode.is_dark {
                    Theme::dark_config()
                } else {
                    Theme::light_config()
                };

                let Some(config) = config.ok() else {
                    log::error!("Failed to get the theme config.");
                    return Task::none();
                };

                if let Err(e) = builder.build().write_entry(&config) {
                    log::error!("Failed to write the theme config: {e}");
                }
                tasks.push(self.update(Message::ReloadColorSchemes));
            }
            Message::SetColorScheme(color_scheme) => {
                let config = ColorScheme::config();
                if let Err(e) = self
                    .color_scheme
                    .set_name(&config, color_scheme.name.clone())
                {
                    log::error!("There was an error selecting the color scheme: {e}");
                }
                if let Err(e) = self
                    .color_scheme
                    .set_path(&config, color_scheme.path.clone())
                {
                    log::error!("There was an error selecting the color scheme: {e}");
                }

                if let Ok(theme) = &color_scheme.read_theme() {
                    log::info!("Color scheme has a theme, setting the theme...");
                    tasks.push(self.update(Message::ImportSuccess(Box::new(theme.clone()))))
                }
            }
            Message::DeleteColorScheme(color_scheme) => {
                if self.color_scheme.name == color_scheme.name
                    && let Some(color_scheme) = self.installed.first()
                {
                    tasks.push(self.update(Message::SetColorScheme(color_scheme.clone())));
                    tasks.push(self.update(Message::ReloadColorSchemes));
                }
                let Some(path) = color_scheme.path else {
                    return Task::none();
                };
                std::fs::remove_file(&path).unwrap_or_else(|e| {
                    log::error!("There was an error deleting the color scheme: {e}")
                });
                tasks.push(self.update(Message::ReloadColorSchemes));
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
                tasks.push(self.update(Message::ReloadColorSchemes));
            }
            Message::FetchAvailableColorSchemes(provider, limit) => {
                if self.offset == 0 {
                    self.status = Status::Loading;
                } else {
                    self.status = Status::LoadingMore;
                }
                self.limit = limit;
                self.offset += self.limit;
                let limit = self.limit;
                let offset = self.offset;
                tasks.push(Task::perform(
                    async move {
                        let url = match provider {
                            ColorSchemeProvider::CosmicThemes => {
                                format!("https://cosmic-themes.org/api/themes/?order=name&limit={}&offset={}", limit, offset)
                            }
                        };

                        let response = reqwest::get(url).await?;
                        let themes: Vec<CosmicTheme> = response.json().await?;
                        let available = themes
                            .into_iter()
                            .map(ColorScheme::from)
                            .collect();
                        Ok(available)
                    },
                    |res: Result<Vec<ColorScheme>, reqwest::Error>| match res {
                        Ok(themes) => Message::SetAvailableColorSchemes(themes),
                        Err(e) => {
                            log::error!("{e}");
                            Message::SetAvailableColorSchemes(vec![])
                        }
                    },
                ));
            }
            Message::SetAvailableColorSchemes(mut available) => {
                self.status = Status::Idle;
                self.available.append(&mut available);
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
                    return Task::none();
                };
                if let Some(path) = path.parent()
                    && let Err(e) = open::that_detached(path)
                {
                    log::error!("There was an error opening that color scheme: {e}")
                }
            }
            Message::ReloadColorSchemes => {
                self.installed = ColorScheme::installed().unwrap_or_default();
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
                        return Task::none();
                    }

                    let Ok(theme_builder) = ron::to_string(&self.theme_builder) else {
                        log::error!("failed to serialize the theme builder");
                        return Task::none();
                    };

                    if let Err(e) = std::fs::write(path, theme_builder) {
                        log::error!("failed to write the file to the themes directory: {e}");
                    }

                    tasks.push(self.update(Message::SetColorScheme(color_scheme)));
                    tasks.push(self.update(Message::ReloadColorSchemes))
                } else {
                    tasks.push(self.update(Message::SaveCurrentColorScheme(None)))
                }
            }
        }
        Task::batch(tasks)
    }

    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        let spacing = cosmic::theme::spacing();
        let active_tab = self.model.active_data::<Tab>().unwrap();
        let title = widget::text::title3(fl!("color-schemes"));
        let tabs = widget::segmented_button::horizontal(&self.model)
            .padding(spacing.space_xxxs)
            .button_alignment(cosmic::iced::Alignment::Center)
            .on_activate(Message::TabSelected);
        let active_tab = match active_tab {
            Tab::Installed => widget::settings::section().add(self.installed_themes()),
            Tab::Available => widget::settings::section().add(self.available_themes()),
        };

        widget::column()
            .push(title)
            .push(tabs)
            .push(active_tab)
            .spacing(spacing.space_xxs)
            .into()
    }

    fn installed_themes<'a>(&'a self) -> Element<'a, Message> {
        if self.installed.is_empty() {
            widget::text("No color schemes installed").into()
        } else {
            widget::responsive(move |size| {
                let spacing = cosmic::theme::spacing();

                let GridMetrics {
                    cols,
                    item_width,
                    column_spacing,
                } = GridMetrics::custom(&spacing, size.width as usize);

                let mut grid = widget::grid();
                let mut col = 0;
                for color_scheme in self.installed.iter() {
                    if col >= cols {
                        grid = grid.insert_row();
                        col = 0;
                    }
                    grid = grid.push(preview::installed(
                        color_scheme,
                        &self.color_scheme,
                        &spacing,
                        item_width,
                    ));
                    col += 1;
                }

                widget::scrollable(
                    grid.column_spacing(column_spacing)
                        .row_spacing(column_spacing),
                )
                .height(Length::Fill)
                .width(Length::Fill)
                .into()
            })
            .into()
        }
    }

    fn available_themes<'a>(&'a self) -> Element<'a, Message> {
        match self.status {
            Status::Idle | Status::LoadingMore => {
                if self.available.is_empty() {
                    widget::text("No color schemes found").into()
                } else {
                    widget::responsive(move |size| {
                        let spacing = cosmic::theme::spacing();

                        let GridMetrics {
                            cols,
                            item_width,
                            column_spacing,
                        } = GridMetrics::custom(&spacing, size.width as usize);

                        let mut grid = widget::grid();
                        let mut col = 0;
                        for color_scheme in self.available.iter() {
                            if col >= cols {
                                grid = grid.insert_row();
                                col = 0;
                            }

                            grid =
                                grid.push(preview::available(color_scheme, &spacing, item_width));
                            col += 1;
                        }

                        widget::scrollable(
                            grid.column_spacing(column_spacing)
                                .row_spacing(column_spacing),
                        )
                        .height(Length::Fill)
                        .width(Length::Fill)
                        .into()
                    })
                    .into()
                }
            }
            Status::Loading => widget::text(fl!("loading")).into(),
        }
    }
}
