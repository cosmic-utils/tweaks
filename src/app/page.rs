use cosmic::widget::Icon;

use crate::fl;

use crate::app::core::icons;

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub enum Page {
    #[default]
    ColorSchemes,
    Dock,
    Panel,
    Layouts,
    Shortcuts,
    Snapshots,
}

impl Default for &Page {
    fn default() -> Self {
        &Page::ColorSchemes
    }
}

impl Page {
    pub fn title(&self) -> String {
        match self {
            Self::ColorSchemes => fl!("color-schemes"),
            Self::Dock => fl!("dock"),
            Self::Panel => fl!("panel"),
            Self::Layouts => fl!("layouts"),
            Self::Shortcuts => fl!("shortcuts"),
            Self::Snapshots => fl!("snapshots"),
        }
    }

    pub fn icon(&self) -> Icon {
        match self {
            Self::ColorSchemes => icons::get_icon("dark-mode-symbolic", 18),
            Self::Dock => icons::get_icon("dock-bottom-symbolic", 18),
            Self::Panel => icons::get_icon("dock-top-symbolic", 18),
            Self::Layouts => icons::get_icon("view-coverflow-symbolic", 18),
            Self::Shortcuts => icons::get_icon("keyboard-symbolic", 18),
            Self::Snapshots => icons::get_icon("snapshots-symbolic", 18),
        }
    }

    pub fn all() -> &'static [Self] {
        &[
            Self::ColorSchemes,
            Self::Dock,
            Self::Panel,
            Self::Layouts,
            Self::Shortcuts,
            Self::Snapshots,
        ]
    }
}
