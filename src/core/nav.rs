use cosmic::{widget::Icon, Element};

use crate::{app, fl, pages};

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

    pub fn view<'a>(&self) -> Element<'a, app::Message> {
        match self {
            NavPage::ColorSchemes => pages::color_schemes::ColorSchemes::default()
                .view()
                .map(Box::new)
                .map(app::Message::ColorSchemes),
            NavPage::Dock => pages::dock::Dock::default().view().map(app::Message::Dock),
            NavPage::Panel => pages::panel::Panel::default()
                .view()
                .map(app::Message::Panel),
        }
    }

    pub fn all() -> &'static [Self] {
        &[Self::ColorSchemes, Self::Dock, Self::Panel]
    }
}
