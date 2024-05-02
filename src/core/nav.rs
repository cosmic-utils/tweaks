use cosmic::{widget::Icon, Element};

use crate::{app, fl, pages};

use super::icons;

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub enum NavPage {
    #[default]
    Dock,
    Panel,
    Themes,
}

impl NavPage {
    pub fn title(&self) -> String {
        match self {
            Self::Dock => fl!("dock"),
            Self::Panel => fl!("panel"),
            Self::Themes => fl!("color-schemes"),
        }
    }

    pub fn icon(&self) -> Icon {
        match self {
            Self::Dock => icons::get_icon("dock-bottom-symbolic", 18),
            Self::Panel => icons::get_icon("dock-top-symbolic", 18),
            Self::Themes => icons::get_icon("dark-mode-symbolic", 18),
        }
    }

    pub fn view<'a>(&self) -> Element<'a, app::Message> {
        match self {
            NavPage::Dock => pages::dock::Dock::default().view().map(app::Message::Dock),
            NavPage::Panel => pages::panel::view().map(app::Message::Panel),
            NavPage::Themes => pages::color_schemes::view().map(app::Message::ColorSchemes),
        }
    }

    pub fn all() -> &'static [Self] {
        &[Self::Dock, Self::Panel, Self::Themes]
    }
}
