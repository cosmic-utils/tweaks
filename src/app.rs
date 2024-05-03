use cosmic::{
    app::{self, Core},
    iced::{Alignment, Command, Length},
    widget::{self, segmented_button},
    Application, Element,
};

use crate::{
    core::nav::NavPage,
    fl,
    pages::{self, color_schemes::ColorSchemes},
};

#[derive(Default)]
pub struct TweakTool {
    core: Core,
    nav_model: segmented_button::SingleSelectModel,
}

#[derive(Debug, Clone)]
pub enum Message {
    Home(pages::home::Message),
    Dock(pages::dock::Message),
    Panel(pages::panel::Message),
    ColorSchemes(pages::color_schemes::Message),
}

impl Application for TweakTool {
    type Executor = cosmic::executor::Default;

    type Flags = ();

    type Message = Message;

    const APP_ID: &'static str = "dev.edfloreshz.CosmicTweakTool";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn header_center(&self) -> Vec<Element<Self::Message>> {
        vec![widget::text::text(fl!("app-title")).into()]
    }

    fn nav_model(&self) -> Option<&widget::nav_bar::Model> {
        Some(&self.nav_model)
    }

    fn on_nav_select(
        &mut self,
        id: widget::nav_bar::Id,
    ) -> cosmic::iced::Command<app::Message<Self::Message>> {
        self.nav_model.activate(id);
        Command::none()
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Command<app::Message<Self::Message>>) {
        let mut nav_model = segmented_button::SingleSelectModel::default();
        for &nav_page in NavPage::all() {
            let id = nav_model
                .insert()
                .icon(nav_page.icon())
                .text(nav_page.title())
                .data::<NavPage>(nav_page)
                .id();

            if nav_page == NavPage::Dock {
                nav_model.activate(id);
            }
        }

        let app = TweakTool { nav_model, core };

        (app, Command::none())
    }

    fn view(&self) -> Element<Self::Message> {
        let spacing = cosmic::theme::active().cosmic().spacing;
        let entity = self.nav_model.active();
        let nav_page = self
            .nav_model
            .data::<NavPage>(entity)
            .unwrap_or(&NavPage::Home);

        widget::column::with_children(vec![nav_page.view()])
            .padding(spacing.space_xs)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }

    fn update(
        &mut self,
        message: Self::Message,
    ) -> cosmic::iced::Command<app::Message<Self::Message>> {
        let mut commands = vec![];
        match message {
            Message::Home(message) => match message {},
            Message::Dock(message) => commands.push(
                pages::dock::Dock::default()
                    .update(message)
                    .map(cosmic::app::Message::App),
            ),
            Message::Panel(message) => commands.push(
                pages::panel::Panel::default()
                    .update(message)
                    .map(cosmic::app::Message::App),
            ),
            Message::ColorSchemes(message) => commands.push(
                ColorSchemes::default()
                    .update(message)
                    .map(Message::ColorSchemes)
                    .map(cosmic::app::Message::App),
            ),
        }
        Command::batch(commands)
    }
}
