use cosmic::widget::Icon;

use crate::fl;

use super::icons;

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub enum NavPage {
    #[default]
    ColorSchemes,
    Dock,
    Panel,
}

impl Default for &NavPage {
    fn default() -> Self {
        &NavPage::ColorSchemes
    }
}

impl NavPage {
    pub fn title(&self) -> String {
        match self {
            Self::ColorSchemes => fl!("color-schemes"),
            Self::Dock => fl!("dock"),
            Self::Panel => fl!("panel"),
        }
    }

    pub fn icon(&self) -> Icon {
        match self {
            Self::ColorSchemes => icons::get_icon("dark-mode-symbolic", 18),
            Self::Dock => icons::get_icon("dock-bottom-symbolic", 18),
            Self::Panel => icons::get_icon("dock-top-symbolic", 18),
        }
    }

    pub fn all() -> &'static [Self] {
        &[Self::ColorSchemes, Self::Dock, Self::Panel]
    }
}
