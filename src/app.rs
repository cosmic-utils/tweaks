use crate::pages::{self, color_schemes::config::ColorScheme};
use ::cosmic::{app::Task, widget, Element};

use cosmic::Cosmic;
use message::Message;

pub mod action;
pub mod context;
mod cosmic;
pub mod dialog;
pub mod flags;
pub mod message;
pub mod page;

pub struct App {
    pub cosmic: Cosmic,
    handler: ::cosmic::cosmic_config::Config,
    config: crate::core::config::TweaksConfig,
    color_schemes: pages::ColorSchemes,
    dock: pages::Dock,
    panel: pages::Panel,
    layouts: pages::Layouts,
    snapshots: pages::Snapshots,
    shorcuts: pages::Shortcuts,
}

impl App {
    fn update_config(&mut self) -> Task<Message> {
        self.color_schemes.theme_builder = ColorScheme::current_theme();
        Task::batch(vec![::cosmic::command::set_theme(
            self.config.app_theme.theme(),
        )])
    }

    fn settings(&self) -> Element<Message> {
        let app_theme_selected = match self.config.app_theme {
            crate::core::config::AppTheme::Dark => 1,
            crate::core::config::AppTheme::Light => 2,
            crate::core::config::AppTheme::System => 0,
        };
        widget::settings::view_column(vec![widget::settings::section()
            .title(crate::fl!("appearance"))
            .add(
                widget::settings::item::builder(crate::fl!("theme")).control(widget::dropdown(
                    &self.cosmic.app_themes,
                    Some(app_theme_selected),
                    Message::AppTheme,
                )),
            )
            .into()])
        .into()
    }
}
