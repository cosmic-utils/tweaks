use crate::pages;

use super::{context::ContextPage, dialog::DialogPage};
use cosmic::iced::keyboard::{Key, Modifiers};

#[derive(Debug, Clone)]
pub enum Message {
    Dock(pages::dock::Message),
    Panel(pages::panel::Message),
    Layouts(pages::layouts::Message),
    Shortcuts(pages::shortcuts::Message),
    Snapshots(pages::snapshots::Message),
    ColorSchemes(Box<pages::color_schemes::Message>),
    ThemePacks(pages::theme_packs::Message),
    DialogUpdate(DialogPage),
    DialogComplete,
    DialogCancel,
    SaveNewColorScheme(String),
    ToggleContextPage(ContextPage),
    ToggleContextDrawer,
    ToggleDialogPage(DialogPage),
    AppTheme(usize),
    Key(Modifiers, Key),
    Modifiers(Modifiers),
    SystemThemeModeChange,
    Open(String),
}
