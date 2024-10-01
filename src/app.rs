use std::collections::VecDeque;

use cosmic::{
    app::{self, Core},
    iced::{Alignment, Command, Length},
    widget::{self, segmented_button},
    Application, Element,
};

use crate::{
    core::nav::NavPage,
    fl,
    pages::{
        self,
        color_schemes::{ColorSchemeProvider, ColorSchemes},
    },
};

pub struct TweakTool {
    core: Core,
    nav_model: segmented_button::SingleSelectModel,
    dialog_pages: VecDeque<DialogPage>,
    dialog_text_input: widget::Id,
    color_schemes: ColorSchemes,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DialogPage {
    New(String),
}

#[derive(Debug, Clone)]
pub enum Message {
    Dock(pages::dock::Message),
    Panel(pages::panel::Message),
    ColorSchemes(Box<pages::color_schemes::Message>),
    OpenSaveDialog,
    DialogUpdate(DialogPage),
    DialogComplete,
    DialogCancel,
    SaveNewColorScheme(String),
}

impl Application for TweakTool {
    type Executor = cosmic::executor::Default;

    type Flags = ();

    type Message = Message;

    const APP_ID: &'static str = "dev.edfloreshz.CosmicTweaks";

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

    fn dialog(&self) -> Option<Element<Self::Message>> {
        let dialog_page = match self.dialog_pages.front() {
            Some(some) => some,
            None => return None,
        };

        let spacing = cosmic::theme::active().cosmic().spacing;

        let dialog = match dialog_page {
            DialogPage::New(name) => widget::dialog(fl!("save-current-color-scheme"))
                .primary_action(
                    widget::button::suggested(fl!("save"))
                        .on_press_maybe(Some(Message::DialogComplete)),
                )
                .secondary_action(
                    widget::button::standard(fl!("cancel")).on_press(Message::DialogCancel),
                )
                .control(
                    widget::column::with_children(vec![
                        widget::text::body(fl!("color-scheme-name")).into(),
                        widget::text_input("", name.as_str())
                            .id(self.dialog_text_input.clone())
                            .on_input(move |name| Message::DialogUpdate(DialogPage::New(name)))
                            .into(),
                    ])
                    .spacing(spacing.space_xxs),
                ),
        };

        Some(dialog.into())
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Command<app::Message<Self::Message>>) {
        log::info!("Starting Cosmic Tweak Tool...");

        let mut nav_model = segmented_button::SingleSelectModel::default();
        for &nav_page in NavPage::all() {
            let id = nav_model
                .insert()
                .icon(nav_page.icon())
                .text(nav_page.title())
                .data::<NavPage>(nav_page)
                .id();

            if nav_page == NavPage::default() {
                nav_model.activate(id);
            }
        }

        let mut app = TweakTool {
            nav_model,
            core,
            dialog_pages: VecDeque::new(),
            dialog_text_input: widget::Id::unique(),
            color_schemes: ColorSchemes::default(),
        };

        let commands = vec![app.update(Message::ColorSchemes(Box::new(
            pages::color_schemes::Message::FetchAvailableColorSchemes(
                ColorSchemeProvider::CosmicThemes,
                app.color_schemes.limit,
            ),
        )))];

        (app, Command::batch(commands))
    }

    fn view(&self) -> Element<Self::Message> {
        let spacing = cosmic::theme::active().cosmic().spacing;
        let entity = self.nav_model.active();
        let nav_page = self.nav_model.data::<NavPage>(entity).unwrap_or_default();

        let view = match nav_page {
            NavPage::ColorSchemes => self
                .color_schemes
                .view()
                .map(Box::new)
                .map(Message::ColorSchemes),
            NavPage::Dock => pages::dock::Dock::default().view().map(Message::Dock),
            NavPage::Panel => pages::panel::Panel::default().view().map(Message::Panel),
        };

        widget::column::with_children(vec![view])
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
            Message::ColorSchemes(message) => match *message {
                pages::color_schemes::Message::SaveCurrentColorScheme(None) => {
                    commands.push(self.update(Message::OpenSaveDialog))
                }
                _ => commands.push(
                    self.color_schemes
                        .update(*message)
                        .map(Box::new)
                        .map(Message::ColorSchemes)
                        .map(cosmic::app::Message::App),
                ),
            },
            Message::SaveNewColorScheme(name) => {
                commands.push(self.update(Message::ColorSchemes(Box::new(
                    pages::color_schemes::Message::SaveCurrentColorScheme(Some(name)),
                ))))
            }
            Message::OpenSaveDialog => {
                self.dialog_pages.push_back(DialogPage::New(String::new()));
                return widget::text_input::focus(self.dialog_text_input.clone());
            }
            Message::DialogUpdate(dialog_page) => {
                self.dialog_pages[0] = dialog_page;
            }
            Message::DialogComplete => {
                if let Some(dialog_page) = self.dialog_pages.pop_front() {
                    match dialog_page {
                        DialogPage::New(name) => {
                            commands.push(self.update(Message::SaveNewColorScheme(name)))
                        }
                    }
                }
            }
            Message::DialogCancel => {
                self.dialog_pages.pop_front();
            }
        }
        Command::batch(commands)
    }
}
