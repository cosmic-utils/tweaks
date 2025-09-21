use action::TweaksAction;
use context::ContextPage;
use cosmic::{
    Application, Core, Element,
    app::{self, Task, context_drawer::ContextDrawer},
    iced, widget,
};
use std::collections::{HashMap, VecDeque};

use crate::app::pages::color_schemes::config::ColorScheme;
use dialog::DialogPage;
use flags::Flags;
use message::Message;

pub mod action;
pub mod context;
pub mod context_drawer;
pub mod core;
pub mod dialog;
pub mod flags;
pub mod footer;
pub mod header;
pub mod init;
pub mod message;
pub mod nav;
pub mod page;
pub mod pages;
pub mod subscription;
pub mod update;
pub mod view;

pub struct App {
    cosmic: Cosmic,
    handler: cosmic::cosmic_config::Config,
    config: core::config::TweaksConfig,
    color_schemes: pages::ColorSchemes,
    dock: pages::Dock,
    panel: pages::Panel,
    layouts: pages::Layouts,
    snapshots: pages::Snapshots,
    shortcuts: pages::ShortcutsPage,
}

pub struct Cosmic {
    core: Core,
    nav_model: widget::segmented_button::SingleSelectModel,
    about: widget::about::About,
    dialog_pages: VecDeque<DialogPage>,
    dialog_text_input: widget::Id,
    key_binds: HashMap<widget::menu::KeyBind, TweaksAction>,
    modifiers: iced::keyboard::Modifiers,
    context_page: ContextPage,
    app_themes: Vec<String>,
}

impl Application for App {
    type Executor = cosmic::executor::Default;

    type Flags = Flags;

    type Message = Message;

    const APP_ID: &'static str = "dev.edfloreshz.CosmicTweaks";

    fn core(&self) -> &Core {
        &self.cosmic.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.cosmic.core
    }

    fn init(core: Core, flags: Self::Flags) -> (Self, app::Task<Self::Message>) {
        Cosmic::init(core, flags)
    }

    fn header_start<'a>(&'a self) -> Vec<Element<'a, Self::Message>> {
        Cosmic::header_start(self)
    }

    fn nav_model(&self) -> Option<&widget::nav_bar::Model> {
        Some(&self.cosmic.nav_model)
    }

    fn on_nav_select(&mut self, id: widget::nav_bar::Id) -> app::Task<Self::Message> {
        Cosmic::on_nav_select(self, id)
    }

    fn context_drawer<'a>(&'a self) -> Option<ContextDrawer<'a, Self::Message>> {
        Cosmic::context_drawer(self)
    }

    fn dialog<'a>(&'a self) -> Option<Element<'a, Self::Message>> {
        Cosmic::dialog(self)
    }

    fn view<'a>(&'a self) -> Element<'a, Self::Message> {
        Cosmic::view(self)
    }

    fn footer<'a>(&'a self) -> Option<Element<'a, Self::Message>> {
        Cosmic::footer(self)
    }

    fn update(&mut self, message: Self::Message) -> app::Task<Self::Message> {
        Cosmic::update(self, message)
    }

    fn subscription(&self) -> cosmic::iced::Subscription<Self::Message> {
        Cosmic::subscription()
    }
}

impl App {
    fn update_config(&mut self) -> Task<Message> {
        self.color_schemes.theme_builder = ColorScheme::current_theme();
        Task::batch(vec![cosmic::command::set_theme(
            self.config.app_theme.theme(),
        )])
    }

    fn settings<'a>(&'a self) -> Element<'a, Message> {
        let app_theme_selected = match self.config.app_theme {
            core::config::AppTheme::Dark => 1,
            core::config::AppTheme::Light => 2,
            core::config::AppTheme::System => 0,
        };
        widget::settings::view_column(vec![
            widget::settings::section()
                .title(crate::fl!("appearance"))
                .add(
                    widget::settings::item::builder(crate::fl!("theme")).control(widget::dropdown(
                        &self.cosmic.app_themes,
                        Some(app_theme_selected),
                        Message::AppTheme,
                    )),
                )
                .into(),
        ])
        .into()
    }
}
