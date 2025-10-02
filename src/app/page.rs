use cosmic::widget::Icon;

use crate::{fl, icon};

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
            Self::ColorSchemes => icon!("dark-mode-symbolic", 18),
            Self::Dock => icon!("dock-bottom-symbolic", 18),
            Self::Panel => icon!("dock-top-symbolic", 18),
            Self::Layouts => icon!("view-coverflow-symbolic", 18),
            Self::Shortcuts => icon!("keyboard-symbolic", 18),
            Self::Snapshots => icon!("snapshots-symbolic", 18),
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
