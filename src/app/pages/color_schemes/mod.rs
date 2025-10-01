use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::bail;
use ashpd::desktop::file_chooser::{FileFilter, SelectedFiles};
use cosmic::cosmic_theme::{Theme, ThemeBuilder, ThemeMode};
use cosmic::{
    Task,
    cosmic_config::{self, Config},
    widget::segmented_button::{self, SingleSelect},
};
use cosmic_config::CosmicConfigEntry;
use cosmic_config::cosmic_config_derive::CosmicConfigEntry;
use serde::{Deserialize, Serialize};

pub mod view;

pub struct ColorSchemes {
    installed: Vec<ColorScheme>,
    available: Vec<ColorScheme>,
    config_writer: Config,
    config: ColorSchemesPageConfig,
    pub model: segmented_button::Model<SingleSelect>,
    pub status: Status,
    saved_color_theme: Option<ColorScheme>,
}

impl ColorSchemes {
    pub fn new() -> (Self, Task<Message>) {
        let is_cache = is_cache_exist();

        let config = match ColorSchemesPageConfig::get_entry(&ColorSchemesPageConfig::config()) {
            Ok(config) => config,
            Err((errors, default)) => {
                log::error!("Failed to load color scheme config: {errors:#?}");
                default
            }
        };

        let s = ColorSchemes {
            installed: installed_system_themes().unwrap(),
            available: if is_cache {
                get_themes_from_cache().unwrap()
            } else {
                vec![]
            },
            saved_color_theme: config.current_config.clone(),
            config,
            config_writer: ColorSchemesPageConfig::config(),
            model: segmented_button::Model::builder()
                .insert(|b| b.text("Installed").data(Tab::Installed).activate())
                .insert(|b| b.text("Available").data(Tab::Available))
                .build(),
            status: if is_cache {
                Status::Idle
            } else {
                Status::Loading
            },
        };

        (
            s,
            Task::perform(async { download_themes().await }, |res| match res {
                Ok(themes) => Message::SetAvailableColorSchemes(themes),
                Err(e) => Message::Error(MessageErrorKind::Fetching, format!("{e}")),
            }),
        )
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
pub enum Message {
    StartImport,
    ImportFilePickerResult(Arc<SelectedFiles>),
    Error(MessageErrorKind, String),
    // currently, the None variant is intercepted in the outer update fn
    SaveCurrentColorScheme(Option<String>),
    InstallColorScheme(ColorScheme),
    SetColorScheme(ColorScheme),
    SetColorSchemeWithRollBack(ColorScheme),
    RevertOldTheme,
    DeleteColorScheme(ColorScheme),
    SetAvailableColorSchemes(Vec<ColorScheme>),
    FetchAvailableColorSchemes,
    OpenFolder(PathBuf),
    OpenLink(String),
    TabSelected(segmented_button::Entity),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageErrorKind {
    Fetching,
    Other,
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
                    self.installed.push(theme.clone());
                    if let Err(e) = apply_theme(theme.theme.clone()) {
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
                if let Err(e) = apply_theme(color_scheme.theme.clone()) {
                    error!("can't apply theme: {e}");
                } else {
                    let _ = self
                        .config
                        .set_current_config(&self.config_writer, Some(color_scheme.clone()));
                    self.saved_color_theme = Some(color_scheme);
                }
            }
            Message::SetColorSchemeWithRollBack(color_scheme) => {
                if let Err(e) = apply_theme(color_scheme.theme.clone()) {
                    error!("can't apply theme: {e}");
                } else {
                    let _ = self
                        .config
                        .set_current_config(&self.config_writer, Some(color_scheme));
                }
            }

            Message::RevertOldTheme => {
                if let Some(old_theme) = &self.saved_color_theme {
                    if let Err(e) = apply_theme(old_theme.theme.clone()) {
                        error!("can't apply theme: {e}");
                    }

                    let _ = self
                        .config
                        .set_current_config(&self.config_writer, Some(old_theme.clone()));
                }
            }
            Message::DeleteColorScheme(color_scheme) => {
                if let Some(path) = &color_scheme.path {
                    let _ = fs::remove_file(path);
                }

                self.installed.retain(|e| e.name != color_scheme.name);
            }
            Message::InstallColorScheme(color_scheme) => match install_theme(color_scheme) {
                Ok(theme) => {
                    self.installed.push(theme);
                }
                Err(e) => {
                    error!("can't install theme: {e}");
                }
            },
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

                        match install_theme(color_scheme) {
                            Ok(theme) => {
                                self.installed.push(theme.clone());

                                let _ = self
                                    .config
                                    .set_current_config(&self.config_writer, Some(theme));
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
        }
        Task::batch(tasks)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ColorScheme {
    pub name: String,
    pub theme: ThemeBuilder,
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
            name,
            theme,
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

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum Source {
    CosmicThemesOrg,
    ImportedFromPath,
    Saved,
    System,
}

#[derive(Debug, Serialize, Clone, Default, Deserialize, PartialEq, CosmicConfigEntry)]
#[version = 1]
pub struct ColorSchemesPageConfig {
    pub current_config: Option<ColorScheme>,
}

const CONFIG_ID: &str = "dev.edfloreshz.CosmicTweaks.ColorScheme";

impl ColorSchemesPageConfig {
    pub fn config() -> Config {
        match Config::new(CONFIG_ID, Self::VERSION) {
            Ok(config) => config,
            Err(err) => panic!("Failed to load config: {}", err),
        }
    }
}

pub async fn download_themes() -> anyhow::Result<Vec<ColorScheme>> {
    #[derive(Deserialize)]
    struct CosmicThemeHelper {
        pub name: String,
        pub ron: String,
        pub author: Option<String>,
        pub link: Option<String>,
        pub downloads: u64,
        pub created: String,
        pub updated: String,
    }

    impl TryFrom<CosmicThemeHelper> for ColorScheme {
        type Error = anyhow::Error;

        fn try_from(value: CosmicThemeHelper) -> Result<Self, Self::Error> {
            Ok(Self {
                name: value.name,
                theme: ron::from_str(&value.ron)?,
                author: value.author,
                link: value.link,
                downloads: Some(value.downloads),
                created: Some(
                    chrono::DateTime::parse_from_rfc3339(&value.created)?.timestamp_millis(),
                ),
                updated: Some(
                    chrono::DateTime::parse_from_rfc3339(&value.created)?.timestamp_millis(),
                ),
                source: Some(Source::CosmicThemesOrg),
                path: None,
            })
        }
    }

    let url = "https://cosmic-themes.org/api/themes/?limit=50000";
    let response = reqwest::get(url).await?;
    let themes: Vec<CosmicThemeHelper> = response.json().await?;

    let themes = themes
        .into_iter()
        .map(ColorScheme::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(themes)
}

fn cache_themes_file_path() -> PathBuf {
    dirs::cache_dir().unwrap().join("tweaks/themes.bin")
}

pub fn is_cache_exist() -> bool {
    cache_themes_file_path().exists()
}

pub fn cache_themes(themes: &Vec<ColorScheme>) -> anyhow::Result<()> {
    let filepath = cache_themes_file_path();

    std::fs::create_dir_all(filepath.parent().unwrap())?;

    let file = File::create(&filepath)?;
    let mut writer = BufWriter::new(file);

    bincode::serde::encode_into_std_write(themes, &mut writer, bincode::config::standard())?;
    Ok(())
}

pub fn get_themes_from_cache() -> anyhow::Result<Vec<ColorScheme>> {
    let filepath = cache_themes_file_path();

    let file = File::open(&filepath)?;
    let mut reader = BufReader::new(file);

    let value = bincode::serde::decode_from_reader(&mut reader, bincode::config::standard())?;
    Ok(value)
}

pub fn apply_theme(theme: ThemeBuilder) -> anyhow::Result<()> {
    let theme_mode_config = ThemeMode::config()?;

    let theme_mode = ThemeMode::get_entry(&theme_mode_config).unwrap();

    let theme_config = if theme_mode.is_dark {
        Theme::dark_config()?
    } else {
        Theme::light_config()?
    };

    theme.build().write_entry(&theme_config)?;

    Ok(())
}

fn get_current_theme() -> anyhow::Result<ThemeBuilder> {
    let theme_mode_config = ThemeMode::config()?;

    let theme_mode = ThemeMode::get_entry(&theme_mode_config).unwrap();

    let theme_builder_config = if theme_mode.is_dark {
        ThemeBuilder::dark_config()?
    } else {
        ThemeBuilder::light_config()?
    };

    let theme_builder = match ThemeBuilder::get_entry(&theme_builder_config) {
        Ok(t) => t,
        Err((errors, t)) => {
            for e in errors {
                log::error!("{e}");
            }
            t
        }
    };

    Ok(theme_builder)
}

fn installed_system_themes() -> anyhow::Result<Vec<ColorScheme>> {
    let mut cosmic_themes = vec![];

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
                source: Some(Source::System),
                downloads: None,
                created: None,
                updated: None,
            };
            cosmic_themes.push(color_scheme);
        }
    }

    Ok(cosmic_themes)
}

fn import_file(f: Arc<SelectedFiles>) -> anyhow::Result<ColorScheme> {
    let Some(f) = f.uris().first() else {
        bail!("no uri")
    };
    if f.scheme() != "file" {
        bail!("scheme != file")
    }
    let Ok(path) = f.to_file_path() else {
        bail!("can't retrieve file path")
    };

    let name = path.file_stem().unwrap().to_str().unwrap().to_string();
    let content = fs::read_to_string(&path)?;

    let builder = ron::de::from_str(&content)?;

    let mut theme = ColorScheme::new(name, builder);

    theme.source = Some(Source::ImportedFromPath);

    let file_name = path.file_name().unwrap();

    let new_file_path = dirs::data_local_dir()
        .unwrap()
        .join("themes/cosmic")
        .join(file_name);

    fs::create_dir_all(new_file_path.parent().unwrap())?;
    fs::write(&new_file_path, &content)?;

    theme.path = Some(new_file_path);

    Ok(theme)
}

fn install_theme(mut theme: ColorScheme) -> anyhow::Result<ColorScheme> {
    let new_file_path = dirs::data_local_dir()
        .unwrap()
        .join("themes/cosmic")
        .join(&theme.name)
        .with_extension("ron");

    fs::create_dir_all(new_file_path.parent().unwrap())?;
    fs::write(&new_file_path, ron::ser::to_string(&theme.theme)?)?;

    theme.path = Some(new_file_path);
    Ok(theme)
}
