use cosmic::{widget::Icon, Element};

use crate::{app, fl, pages};

use super::icons;

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub enum NavPage {
    #[default]
    Home,
    Dock,
    Panel,
    Themes,
}

impl NavPage {
    pub fn title(&self) -> String {
        match self {
            Self::Home => fl!("home"),
            Self::Dock => fl!("dock"),
            Self::Panel => fl!("panel"),
            Self::Themes => fl!("color-schemes"),
        }
    }

    pub fn icon(&self) -> Icon {
        match self {
            Self::Home => icons::get_icon("view-switcher-symbolic", 18),
            Self::Dock => icons::get_icon("dock-bottom-symbolic", 18),
            Self::Panel => icons::get_icon("dock-top-symbolic", 18),
            Self::Themes => icons::get_icon("dark-mode-symbolic", 18),
        }
    }

    pub fn view<'a>(&self) -> Element<'a, app::Message> {
        match self {
            NavPage::Home => pages::home::view().map(app::Message::Home),
            NavPage::Dock => pages::dock::Dock::default().view().map(app::Message::Dock),
            NavPage::Panel => pages::panel::Panel::default()
                .view()
                .map(app::Message::Panel),
            NavPage::Themes => pages::color_schemes::ColorSchemes::default()
                .view()
                .map(Box::new)
                .map(app::Message::ColorSchemes),
        }
    }

    pub fn all() -> &'static [Self] {
        &[Self::Home, Self::Dock, Self::Panel, Self::Themes]
    }
}
