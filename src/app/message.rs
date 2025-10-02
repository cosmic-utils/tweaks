use crate::app::{
    core::config::TweaksConfig,
    pages::{self, layouts::preview::LayoutPreview},
};

use super::{context::ContextPage, dialog::DialogPage};
use cosmic::{
    cosmic_theme::ThemeMode,
    iced::keyboard::{Key, Modifiers},
    widget,
};

#[derive(Debug, Clone)]
pub enum Message {
    Dock(pages::dock::Message),
    Panel(pages::panel::Message),
    Layouts(pages::layouts::Message),
    Shortcuts(pages::shortcuts::Message),
    Snapshots(pages::snapshots::Message),
    ColorSchemes(Box<pages::color_schemes::Message>),
    UpdatePanelLayoutPosition(widget::segmented_button::Entity, String, LayoutPreview),
    UpdateDockLayoutPosition(widget::segmented_button::Entity, String, LayoutPreview),
    DialogUpdate(DialogPage),
    DialogComplete,
    DialogCancel,
    SaveNewColorScheme(String),
    ToggleContextPage(ContextPage),
    ToggleContextDrawer,
    ToggleDialogPage(DialogPage),
    Key(Modifiers, Key),
    Modifiers(Modifiers),
    SystemThemeModeChange(ThemeMode),
    Open(String),
    Settings(SettingsMessage),
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    AppTheme(usize),
    ConfigUpdate(TweaksConfig),
}
