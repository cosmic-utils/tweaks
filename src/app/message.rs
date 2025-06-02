use crate::app::pages::{self, layouts::preview::LayoutPreview};

use super::{context::ContextPage, dialog::DialogPage};
use cosmic::{
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
    AppTheme(usize),
    Key(Modifiers, Key),
    Modifiers(Modifiers),
    SystemThemeModeChange,
    Open(String),
}
