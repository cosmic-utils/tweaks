pub mod theme_pack;

pub use theme_pack::ThemePack;

use crate::fl;
use cosmic::{
    cosmic_config::CosmicConfigEntry,
    cosmic_theme::ThemeBuilder,
    dialog::file_chooser::{self, FileFilter},
    iced, widget, Element, Task,
};
use std::{fs, path::PathBuf};
use url::Url;

// File extension for COSMIC theme packs
pub const THEME_PACK_EXTENSION: &str = "ctp";

#[derive(Debug, Clone)]
pub enum Message {
    ThemeNameChanged(String),
    ThemeAuthorChanged(String),
    ThemeDescriptionChanged(String),
    ExportThemePack,
    ImportThemePack,
    ThemePackImported(Option<PathBuf>),
    SaveThemePack,
    CancelSaveThemePack,
    SelectTheme(usize),
    ApplyThemePack,
    DeleteThemePack,
    RefreshThemes,
    FileDialogCancelled,
    FileDialogError(String),
    FileDialogSelected(Url),
}

#[derive(Debug, Default)]
pub struct ThemePacks {
    theme_name: String,
    theme_description: String,
    theme_author: String,
    available_themes: Vec<(String, PathBuf)>,
    selected_theme: Option<usize>,
    save_dialog_open: bool,
}

impl ThemePacks {
    pub fn new() -> Self {
        let mut this = Self::default();
        this.refresh_themes();
        this
    }

    pub fn refresh_themes(&mut self) {
        self.available_themes.clear();

        // Debug log the theme directory
        let theme_dir = Self::get_theme_dir();
        log::info!("Looking for theme packs in: {}", theme_dir.display());

        for path in Self::list_available_themes() {
            if let Some(file_name) = path.file_stem() {
                if let Some(name) = file_name.to_str() {
                    log::info!("Found theme pack: {} at {}", name, path.display());
                    self.available_themes.push((name.to_string(), path));
                }
            }
        }

        log::info!("Total theme packs found: {}", self.available_themes.len());
    }

    /// Get the default theme pack directory
    pub fn get_theme_dir() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_default();
        path.push("cosmic");
        path.push("theme_packs");

        if !path.exists() {
            let _ = fs::create_dir_all(&path);
        }

        path
    }

    /// List available theme packs in the default directory
    pub fn list_available_themes() -> Vec<PathBuf> {
        let dir = Self::get_theme_dir();
        let mut themes = Vec::new();

        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file()
                    && path
                        .extension()
                        .is_some_and(|ext| ext == THEME_PACK_EXTENSION)
                {
                    themes.push(path);
                }
            }
        }

        themes
    }

    pub fn update(&mut self, message: Message) -> Task<crate::app::message::Message> {
        match message {
            Message::ThemeNameChanged(name) => self.theme_name = name,
            Message::ThemeDescriptionChanged(desc) => self.theme_description = desc,
            Message::ThemeAuthorChanged(author) => self.theme_author = author,
            Message::ExportThemePack => self.save_dialog_open = true,
            Message::SaveThemePack => {
                self.save_dialog_open = false;

                if self.theme_name.is_empty() {
                    return Task::none();
                }

                // Create theme pack from current configuration
                let theme_pack = match theme_pack::create_theme_pack(
                    self.theme_name.clone(),
                    self.theme_author.clone(),
                    self.theme_description.clone(),
                ) {
                    Ok(pack) => pack,
                    Err(e) => {
                        log::error!("Failed to create theme pack: {e}");
                        return Task::none();
                    }
                };

                // Save to file
                let mut path = Self::get_theme_dir();
                path.push(format!("{}.{}", self.theme_name, THEME_PACK_EXTENSION));

                if let Err(e) = save_theme_pack(&theme_pack, &path) {
                    log::error!("Failed to save theme pack: {e}");
                }

                self.refresh_themes();
            }
            Message::CancelSaveThemePack => self.save_dialog_open = false,
            Message::SelectTheme(idx) => self.selected_theme = Some(idx),
            Message::ApplyThemePack => {
                if let Some(idx) = self.selected_theme {
                    if let Some((_, path)) = self.available_themes.get(idx) {
                        if let Ok(theme_pack) = load_theme_pack(path) {
                            if let Err(e) = apply_theme_pack(&theme_pack) {
                                log::error!("Failed to apply theme pack: {e}");
                            }
                        }
                    }
                }
            }
            Message::DeleteThemePack => {
                if let Some(idx) = self.selected_theme {
                    if let Some((_, path)) = self.available_themes.get(idx) {
                        let _ = fs::remove_file(path);
                        self.refresh_themes();
                        self.selected_theme = None;
                    }
                }
            }
            Message::RefreshThemes => self.refresh_themes(),
            Message::ImportThemePack => {
                // Use XDG portal file dialog for selecting theme packs
                return cosmic::task::future(async move {
                    log::info!("Opening file dialog for theme pack import");

                    // Create a filter for .ctp files
                    let filter = FileFilter::new("COSMIC Theme Packs")
                        .glob(&format!("*.{}", THEME_PACK_EXTENSION));

                    let dialog = file_chooser::open::Dialog::new()
                        .title("Import Theme Pack")
                        .filter(filter);

                    match dialog.open_file().await {
                        Ok(response) => crate::app::message::Message::ThemePacks(
                            Message::FileDialogSelected(response.url().to_owned()),
                        ),
                        Err(file_chooser::Error::Cancelled) => {
                            crate::app::message::Message::ThemePacks(Message::FileDialogCancelled)
                        }
                        Err(why) => crate::app::message::Message::ThemePacks(
                            Message::FileDialogError(format!("File dialog error: {}", why)),
                        ),
                    }
                });
            }
            Message::FileDialogCancelled => {
                log::info!("Theme pack import cancelled");
            }
            Message::FileDialogError(error) => {
                log::error!("Theme pack import error: {}", error);
            }
            Message::FileDialogSelected(url) => {
                log::info!("Selected file from dialog: {}", url);

                // Convert URL to path
                if let Ok(path) = url.to_file_path() {
                    return Task::perform(async move { Some(path) }, |result| {
                        crate::app::message::Message::ThemePacks(Message::ThemePackImported(result))
                    });
                } else {
                    log::error!("Invalid file path from URL: {}", url);
                }
            }
            Message::ThemePackImported(maybe_path) => {
                if let Some(path) = maybe_path {
                    // Import theme pack from the selected file
                    if let Ok(theme_pack) = load_theme_pack(&path) {
                        // Save it to the theme packs directory
                        let mut new_path = Self::get_theme_dir();
                        new_path.push(format!("{}.{}", theme_pack.name, THEME_PACK_EXTENSION));

                        if let Err(e) = save_theme_pack(&theme_pack, &new_path) {
                            log::error!("Failed to save imported theme pack: {e}");
                        } else {
                            log::info!(
                                "Successfully imported theme pack: {} to {}",
                                theme_pack.name,
                                new_path.display()
                            );
                        }
                    } else {
                        log::error!("Failed to load theme pack from: {}", path.display());
                    }

                    // Refresh the list
                    self.refresh_themes();
                }
            }
        }

        Task::none()
    }

    pub fn view(&self) -> Element<Message> {
        let spacing = cosmic::theme::spacing();

        // Create theme form
        let create_theme_form = widget::column()
            .push(widget::text::title4(fl!("create-new-theme")))
            .push(
                widget::column()
                    .push(widget::text::body(fl!("name")))
                    .push(
                        widget::text_input("", &self.theme_name)
                            .on_input(Message::ThemeNameChanged)
                            .padding(spacing.space_xxs),
                    )
                    .spacing(spacing.space_xxs),
            )
            .push(
                widget::column()
                    .push(widget::text::body(fl!("author")))
                    .push(
                        widget::text_input("", &self.theme_author)
                            .on_input(Message::ThemeAuthorChanged)
                            .padding(spacing.space_xxs),
                    )
                    .spacing(spacing.space_xxs),
            )
            .push(
                widget::column()
                    .push(widget::text::body(fl!("description")))
                    .push(
                        widget::text_input("", &self.theme_description)
                            .on_input(Message::ThemeDescriptionChanged)
                            .padding(spacing.space_xxs),
                    )
                    .spacing(spacing.space_xxs),
            )
            .push(
                widget::row()
                    .push(widget::horizontal_space())
                    .push(
                        widget::button::standard(fl!("export-theme"))
                            .on_press(Message::ExportThemePack),
                    )
                    .spacing(spacing.space_xxs),
            )
            .spacing(spacing.space_m)
            .padding(spacing.space_m);

        // Create form container
        let create_form_container = widget::container(create_theme_form)
            .padding(spacing.space_m)
            .style(|_| widget::container::Style::default());

        // Available themes section with import button
        let available_themes_header = widget::row()
            .push(widget::text::title4(fl!("available-themes")))
            .push(widget::horizontal_space())
            .push(
                widget::button::standard(fl!("import-color-scheme")) // Reuse existing translation
                    .on_press(Message::ImportThemePack),
            )
            .width(iced::Length::Fill);

        let available_themes_section = if self.available_themes.is_empty() {
            widget::column()
                .push(available_themes_header)
                .push(widget::text::body(fl!("no-themes-available")))
                .spacing(spacing.space_m)
                .padding(spacing.space_m)
                .width(iced::Length::Fill)
        } else {
            let mut column = widget::column()
                .push(available_themes_header)
                .spacing(spacing.space_m)
                .padding(spacing.space_m)
                .width(iced::Length::Fill);

            // Create a list of available themes
            for (idx, (name, _)) in self.available_themes.iter().enumerate() {
                let theme_row = widget::row()
                    .push(widget::text::body(name.clone()).width(iced::Length::Fill))
                    .push(widget::radio(
                        "",
                        idx,
                        self.selected_theme,
                        Message::SelectTheme,
                    ))
                    .spacing(spacing.space_m);

                column = column.push(theme_row);
            }

            // Add action buttons for selected theme
            if self.selected_theme.is_some() {
                column = column.push(
                    widget::row()
                        .push(widget::horizontal_space())
                        .push(
                            widget::button::standard(fl!("apply"))
                                .on_press(Message::ApplyThemePack),
                        )
                        .push(
                            widget::button::destructive(fl!("delete"))
                                .on_press(Message::DeleteThemePack),
                        )
                        .spacing(spacing.space_xxs),
                );
            }

            column
        };

        let available_themes_container = widget::container(available_themes_section)
            .padding(spacing.space_m)
            .style(|_| widget::container::Style::default());

        // If save dialog is open, show the confirmation dialog
        if self.save_dialog_open {
            widget::container(
                widget::column()
                    .push(widget::text::title4(fl!("save-theme-confirmation")))
                    .push(
                        widget::row()
                            .push(widget::horizontal_space())
                            .push(
                                widget::button::standard(fl!("cancel"))
                                    .on_press(Message::CancelSaveThemePack),
                            )
                            .push(
                                widget::button::suggested(fl!("save"))
                                    .on_press(Message::SaveThemePack),
                            )
                            .spacing(spacing.space_xxs),
                    )
                    .spacing(spacing.space_m)
                    .padding(spacing.space_m),
            )
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .style(|_| widget::container::Style::default())
            .into()
        } else {
            // Main layout with content section
            let content = widget::column()
                .push(create_form_container)
                .push(available_themes_container)
                .spacing(spacing.space_m);

            widget::settings::view_column(vec![widget::settings::section()
                .title(fl!("theme-packs"))
                .add(content)
                .into()])
            .into()
        }
    }
}

fn save_theme_pack(
    theme_pack: &ThemePack,
    path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let json = ron::to_string(theme_pack)?;
    std::fs::write(path, json)?;

    log::info!("Theme pack saved to: {}", path.display());

    Ok(())
}

fn load_theme_pack(path: &PathBuf) -> Result<ThemePack, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let theme_pack: ThemePack = ron::from_str(&content)?;
    Ok(theme_pack)
}

fn apply_theme_pack(theme_pack: &ThemePack) -> Result<(), Box<dyn std::error::Error>> {
    // Apply the color scheme by saving it to the appropriate location
    log::info!("Applying color scheme from theme pack: {}", theme_pack.name);

    // Try to parse the theme from the color scheme
    let color_scheme = &theme_pack.color_scheme.theme_builder_ron;

    // Try to parse the theme_builder_ron as a ThemeBuilder
    let theme_builder: ThemeBuilder = match ron::from_str(color_scheme) {
        Ok(builder) => builder,
        Err(e) => {
            // Parsing directly fails, try to parse as a ColorScheme
            match ron::from_str::<crate::pages::color_schemes::config::ColorScheme>(color_scheme) {
                Ok(color_scheme) => color_scheme.theme,
                Err(_) => {
                    return Err(Box::new(e));
                }
            }
        }
    };

    // Save color scheme for reference
    let mut theme_pack_path = dirs::data_local_dir().unwrap_or_default();
    theme_pack_path.push("theme_packs/cosmic");

    if !theme_pack_path.exists() {
        fs::create_dir_all(&theme_pack_path)?;
    }

    theme_pack_path.push(format!("{}.ron", theme_pack.name));

    // Save the theme pack to the theme pack directory (for reference only)
    fs::write(&theme_pack_path, &theme_pack.color_scheme.theme_builder_ron)?;

    // Get current theme mode to determine which config to update
    let theme_mode_config = cosmic::cosmic_theme::ThemeMode::config().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to get theme mode config: {e}"),
        )
    })?;

    let theme_mode =
        cosmic::cosmic_theme::ThemeMode::get_entry(&theme_mode_config).map_err(|(e, _)| {
            let error_str = e
                .into_iter()
                .map(|err| err.to_string())
                .collect::<Vec<_>>()
                .join(", ");

            std::io::Error::new(std::io::ErrorKind::Other, error_str)
        })?;

    // Update the corresponding config based on the theme mode
    let config = if theme_mode.is_dark {
        // Get dark theme config
        cosmic::cosmic_theme::Theme::dark_config().map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to get dark theme config: {e}"),
            )
        })?
    } else {
        // Get light theme config
        cosmic::cosmic_theme::Theme::light_config().map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to get light theme config: {e}"),
            )
        })?
    };

    // Write the theme
    if let Err(e) = theme_builder.build().write_entry(&config) {
        log::error!("Failed to write the theme config: {e}");
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to write them config: {e}"),
        )));
    }

    log::info!("Applied color scheme from theme pack: {}", theme_pack.name);

    // Apply the panel config
    log::info!(
        "Applying panel config from theme pack: {}",
        &theme_pack.name
    );

    // Apply the Panel Schema as the theme pack layout
    if let Err(e) = cosmic_ext_config_templates::load_template(theme_pack.layout.clone()) {
        log::error!("Failed to apply layout schema: {e}");
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to load theme pack layout template: {e}"),
        )));
    }

    log::info!("Successfully applied layout from theme pack");

    Ok(())
}
