use std::sync::Arc;

use ashpd::desktop::file_chooser::SelectedFiles;
use cosmic::cosmic_theme::ThemeBuilder;

use crate::pages::{
    color_schemes::{config::ColorScheme, ColorSchemes}, 
    layouts::config::{Layout, LayoutsConfig}
};

#[derive(Debug, Clone)]
pub enum Message {
    // Color Scheme Messages
    StartColorSchemeImport,
    ColorSchemeImportError,
    ColorSchemeImportFile(Arc<SelectedFiles>),
    ColorSchemeImportSuccess(Box<ThemeBuilder>),
    SaveCurrentColorScheme(Option<String>),
    SetColorScheme(ColorScheme),
    DeleteColorScheme(ColorScheme),
    InstallColorScheme(ColorScheme),
    OpenContainingFolder(ColorScheme),
    ReloadColorSchemes,
    OpenAvailableThemes,
    // Dock Messages
    SetDockPadding(u32),
    SetDockSpacing(u32),
    // Panel Messages
    ShowPanel(bool),
    ForceIcons(bool),
    SetPanelPadding(u32),
    SetPanelSpacing(u32),
    // Layout Messages
    ApplyLayout(Layout),
    SelectLayout(Layout),
    DeleteLayout,
    SaveCurrentLayout(String),
    // Theme Pack Messages
    ApplyThemePack((Layout, ColorSchemes)),
    SelectThemePack((Layout, ColorSchemes)),
    SaveThemePack,
    DeleteThemePack,
    // Universal Messages
    OpenSaveDialog,
    OpenLink(Option<String>),
}