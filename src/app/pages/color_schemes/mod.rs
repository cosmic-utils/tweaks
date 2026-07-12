use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use ashpd::desktop::file_chooser::{FileFilter, SelectedFiles};
use cosmic::cosmic_theme::{Theme, ThemeMode};
use cosmic::{
    Task,
    cosmic_config::{self, Config},
    widget::segmented_button::{self, SingleSelect},
};
use cosmic_config::CosmicConfigEntry;
use nucleo::{
    Matcher, Utf32Str,
    pattern::{Atom, AtomKind, CaseMatching, Normalization},
};
use uuid::Uuid;

use crate::app::core::reset::reset_cosmic_config;
use crate::app::pages::color_schemes::config::ColorSchemesPageConfig;
use crate::app::pages::color_schemes::models::{
    ColorScheme, ColorSchemeKey, MessageErrorKind, SortBy, Source, Status, Tab,
};
use crate::app::pages::color_schemes::network::download_themes;
use crate::app::pages::color_schemes::storage::{
    TempColorScheme, cache_themes, get_themes_from_cache, install_theme, installed_system_themes,
    validate_cache_integrity,
};
use crate::app::pages::color_schemes::theme::{apply_theme, get_current_theme, import_file};

pub mod config;
pub mod models;
pub mod network;
pub mod storage;
pub mod theme;
pub mod view;

pub struct ColorSchemes {
    installed: HashMap<String, ColorScheme>,
    available: Vec<ColorScheme>,
    themes: HashMap<Uuid, Theme>,
    config_writer: Config,
    config: ColorSchemesPageConfig,
    model: segmented_button::Model<SingleSelect>,
    status: Status,
    saved_color_theme: Option<ColorScheme>,
    theme_mode: ThemeMode,
    query: String,
    sort_by: SortBy,
    needle: Option<Atom>,
    matcher: RefCell<Matcher>,
}

#[derive(Debug, Clone)]
pub enum Message {
    StartImport,
    ImportFilePickerResult(Arc<SelectedFiles>),
    Error(MessageErrorKind, String),
    // currently, the None variant is intercepted in the outer update fn
    SaveCurrentColorScheme(Option<String>),
    InstallColorScheme(ColorSchemeKey),
    SetColorScheme(ColorSchemeKey),
    SetColorSchemeWithRollBack(ColorSchemeKey),
    RevertOldTheme,
    DeleteColorScheme(ColorSchemeKey),
    SetAvailableColorSchemes(Vec<TempColorScheme>),
    FetchAvailableColorSchemes,
    OpenFolder(PathBuf),
    OpenLink(String),
    TabSelected(segmented_button::Entity),
    ToggleDarkMode(bool),
    SortBy(SortBy),
    Query(String),
    Reset,
}

impl ColorSchemes {
    pub fn new() -> (Self, Task<Message>) {
        let config = match ColorSchemesPageConfig::get_entry(&ColorSchemesPageConfig::config()) {
            Ok(config) => config,
            Err((errors, default)) => {
                log::error!("Failed to load color scheme config: {errors:#?}");
                default
            }
        };

        let mut need_fetching = true;

        let available = if validate_cache_integrity() {
            match get_themes_from_cache() {
                Ok(themes) => {
                    need_fetching = false;
                    themes
                }
                Err(e) => {
                    error!("can't load themes from cache: {e}");
                    vec![]
                }
            }
        } else {
            vec![]
        };

        let installed = installed_system_themes().unwrap();
        let mut themes = HashMap::new();

        for theme in installed.iter().chain(available.iter()) {
            themes
                .entry(theme.id)
                .or_insert(theme.theme_builder.clone().build().clone());
        }

        let s = ColorSchemes {
            installed: installed.into_iter().map(|e| (e.name.clone(), e)).collect(),
            available,
            themes,
            saved_color_theme: config.current_config.clone(),
            config,
            config_writer: ColorSchemesPageConfig::config(),
            model: segmented_button::Model::builder()
                .insert(|b| b.text(fl!("installed")).data(Tab::Installed).activate())
                .insert(|b| b.text(fl!("available")).data(Tab::Available))
                .build(),
            status: if need_fetching {
                Status::Loading
            } else {
                Status::Idle
            },
            theme_mode: {
                let theme_mode_config = ThemeMode::config().unwrap();
                ThemeMode::get_entry(&theme_mode_config).unwrap()
            },
            query: String::new(),
            sort_by: SortBy::default(),
            needle: None,
            matcher: Matcher::new(nucleo::Config::DEFAULT).into(),
        };

        let mut tasks = vec![];

        if need_fetching {
            tasks.push(Task::perform(
                async { download_themes().await },
                |res| match res {
                    Ok(themes) => Message::SetAvailableColorSchemes(themes),
                    Err(e) => Message::Error(MessageErrorKind::Fetching, format!("{e}")),
                },
            ));
        }

        (s, Task::batch(tasks))
    }

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
                        Message::ImportFilePickerResult(Arc::new(f))
                    } else {
                        Message::Error(
                            MessageErrorKind::Other,
                            "failed to select a file for importing a custom theme.".into(),
                        )
                    }
                },
            )),
            Message::Error(kind, m) => {
                if kind == MessageErrorKind::Fetching {
                    self.status = Status::Idle;
                }

                // TODO Error toast?
                error!("{m}");
            }
            Message::ImportFilePickerResult(f) => match import_file(f) {
                Ok(theme) => {
                    self.installed.insert(theme.name.clone(), theme.clone());
                    self.themes
                        .entry(theme.id)
                        .or_insert(theme.theme_builder.clone().build().clone());
                    if let Err(e) = apply_theme(&theme.theme_builder) {
                        error!("can't apply theme: {e}");
                    } else {
                        let _ = self
                            .config
                            .set_current_config(&self.config_writer, Some(theme.clone()));
                        self.saved_color_theme = Some(theme);
                    }
                }
                Err(e) => {
                    error!("can't import file: {e}");
                }
            },
            Message::SetColorScheme(color_scheme) => {
                let color_scheme = self.get(color_scheme).clone();
                if let Err(e) = apply_theme(&color_scheme.theme_builder) {
                    error!("can't apply theme: {e}");
                } else {
                    let _ = self
                        .config
                        .set_current_config(&self.config_writer, Some(color_scheme.clone()));
                    self.saved_color_theme = Some(color_scheme);
                }
            }
            Message::SetColorSchemeWithRollBack(color_scheme) => {
                let color_scheme = self.get(color_scheme);
                if let Err(e) = apply_theme(&color_scheme.theme_builder) {
                    error!("can't apply theme: {e}");
                } else {
                    let _ = self
                        .config
                        .set_current_config(&self.config_writer, Some(color_scheme.clone()));
                }
            }
            Message::RevertOldTheme => {
                if let Some(old_color_scheme) = &self.saved_color_theme {
                    if let Err(e) = apply_theme(&old_color_scheme.theme_builder) {
                        error!("can't apply theme: {e}");
                    }

                    let _ = self
                        .config
                        .set_current_config(&self.config_writer, Some(old_color_scheme.clone()));
                }
            }
            Message::DeleteColorScheme(color_scheme) => {
                let color_scheme = self.get(color_scheme).clone();
                if let Some(path) = &color_scheme.path {
                    let _ = fs::remove_file(path);
                }

                self.installed.remove(&color_scheme.name);
            }
            Message::InstallColorScheme(color_scheme) => {
                let color_scheme = self.get(color_scheme);
                match install_theme(color_scheme.clone(), false) {
                    Ok(theme) => {
                        self.installed.insert(theme.name.clone(), theme);
                    }
                    Err(e) => {
                        error!("can't install theme: {e}");
                    }
                }
            }
            Message::FetchAvailableColorSchemes => {
                self.status = Status::Loading;
                tasks.push(Task::perform(
                    async { download_themes().await },
                    |res| match res {
                        Ok(themes) => Message::SetAvailableColorSchemes(themes),
                        Err(e) => Message::Error(MessageErrorKind::Fetching, format!("{e}")),
                    },
                ));
            }
            Message::SetAvailableColorSchemes(available) => {
                self.status = Status::Idle;

                for theme in &available {
                    self.themes.entry(theme.id).or_insert(theme.theme.clone());
                }

                let available: Vec<ColorScheme> =
                    available.into_iter().map(TempColorScheme::into).collect();

                if let Err(e) = cache_themes(&available) {
                    error!("can't cache themes: {e}");
                }

                self.available = available;
            }
            Message::OpenLink(link) => {
                if let Err(e) = open::that_detached(link) {
                    error!("There was an error opening the link: {e}")
                }
            }
            Message::OpenFolder(path) => {
                if let Some(path) = path.parent()
                    && let Err(e) = open::that_detached(path)
                {
                    error!("There was an error opening that color scheme: {e}")
                }
            }
            Message::SaveCurrentColorScheme(name) => {
                let name = name.unwrap();

                match get_current_theme() {
                    Ok(theme_builder) => {
                        let mut color_scheme = ColorScheme::new(name, theme_builder);
                        color_scheme.source = Some(Source::Saved);

                        match install_theme(color_scheme, false) {
                            Ok(color_scheme) => {
                                self.installed
                                    .insert(color_scheme.name.clone(), color_scheme.clone());
                                self.themes
                                    .entry(color_scheme.id)
                                    .or_insert(color_scheme.theme_builder.clone().build().clone());

                                let _ = self
                                    .config
                                    .set_current_config(&self.config_writer, Some(color_scheme));
                            }
                            Err(e) => {
                                error!("can't install theme: {e}");
                            }
                        }
                    }
                    Err(e) => {
                        error!("can't get current theme: {e}");
                    }
                }
            }
            Message::ToggleDarkMode(dark) => {
                let theme_mode_config = ThemeMode::config().unwrap();
                let _ = self.theme_mode.set_is_dark(&theme_mode_config, dark);
            }
            Message::SortBy(sort_by) => self.sort_by = sort_by,
            Message::Query(query) => self.set_query(query),
            Message::Reset => {
                reset_cosmic_config("com.system76.CosmicTheme.Dark");
                reset_cosmic_config("com.system76.CosmicTheme.Dark.Builder");
                reset_cosmic_config("com.system76.CosmicTheme.Light");
                reset_cosmic_config("com.system76.CosmicTheme.Light.Builder");
                let (new_self, task) = ColorSchemes::new();
                *self = new_self;
                return task;
            }
        }
        Task::batch(tasks)
    }

    pub fn set_theme_mode(&mut self, mode: ThemeMode) {
        self.theme_mode = mode;
    }

    fn get(&self, key: ColorSchemeKey) -> &ColorScheme {
        match key {
            ColorSchemeKey::Installed(name) => self.installed.get(&name).unwrap(),
            ColorSchemeKey::Available(index) => &self.available[index],
        }
    }

    fn set_query(&mut self, query: String) {
        if query.is_empty() {
            self.needle.take();
        } else {
            let atom = Atom::new(
                &query,
                CaseMatching::Smart,
                Normalization::Smart,
                AtomKind::Substring,
                true,
            );

            self.needle.replace(atom);
        }

        self.query = query;
    }

    fn values<'a>(&'a self) -> Box<dyn Iterator<Item = (ColorSchemeKey, &'a ColorScheme)> + 'a> {
        let mut data: Box<dyn Iterator<Item = (ColorSchemeKey, &ColorScheme)>> =
            match self.model.active_data::<Tab>().unwrap() {
                Tab::Installed => Box::new(
                    self.installed
                        .iter()
                        .map(|(a, b)| (ColorSchemeKey::Installed(a.clone()), b)),
                ),
                Tab::Available => Box::new(
                    self.available
                        .iter()
                        .enumerate()
                        .map(|(a, b)| (ColorSchemeKey::Available(a), b)),
                ),
            };

        if let Some(atom) = &self.needle {
            data = Box::new(data.filter(|c| {
                let mut buf = Vec::new();

                let haystack = Utf32Str::new(&c.1.name, &mut buf);

                let mut indices = Vec::new();

                let _res = atom.indices(haystack, &mut self.matcher.borrow_mut(), &mut indices);

                !indices.is_empty()
            }));
        };

        let mut vec = data.collect::<Vec<_>>();

        vec.sort_by(|a, b| self.sort_by.compare(a.1, b.1));

        Box::new(vec.into_iter())
    }
}
