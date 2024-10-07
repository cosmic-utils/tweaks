use std::collections::{HashMap, VecDeque};

use crate::{
    core::{config_manager::ConfigManager, nav::NavPage},
    fl,
    pages::{
        self,
        color_schemes::{config::ColorScheme, preview, ColorSchemeProvider, ColorSchemes},
    },
    settings::{AppTheme, TweaksSettings},
};
use cosmic::{
    app::{self, Core},
    cosmic_config,
    iced::{Alignment, Command, Length},
    widget::{
        self,
        menu::{self, KeyBind},
        segmented_button,
    },
    Application, ApplicationExt, Apply, Element,
};
use key_bind::key_binds;
use pages::color_schemes::providers::cosmic_themes::CosmicTheme;

mod key_bind;

pub struct TweakTool {
    core: Core,
    nav_model: segmented_button::SingleSelectModel,
    dialog_pages: VecDeque<DialogPage>,
    dialog_text_input: widget::Id,
    key_binds: HashMap<KeyBind, Action>,
    color_schemes: ColorSchemes,
    context_page: ContextPage,
    app_themes: Vec<String>,
    config_manager: ConfigManager,
    available: Vec<ColorScheme>,
    status: Status,
    limit: usize,
    offset: usize,
}

pub enum Status {
    Idle,
    Loading,
    LoadingMore,
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
    ToggleContextPage(ContextPage),
    LaunchUrl(String),
    AppTheme(usize),
    FetchAvailableColorSchemes(ColorSchemeProvider, usize),
    SetAvailableColorSchemes(Vec<ColorScheme>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ContextPage {
    AvailableThemes,
    Settings,
    About,
}

impl ContextPage {
    fn title(&self) -> String {
        match self {
            Self::About => fl!("about"),
            Self::Settings => fl!("settings"),
            Self::AvailableThemes => fl!("available"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    About,
    Settings,
}

impl cosmic::widget::menu::Action for Action {
    type Message = Message;
    fn message(&self) -> Self::Message {
        match self {
            Action::About => Message::ToggleContextPage(ContextPage::About),
            Action::Settings => Message::ToggleContextPage(ContextPage::Settings),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Flags {
    pub config_handler: Option<cosmic_config::Config>,
    pub config: TweaksSettings,
}

impl Application for TweakTool {
    type Executor = cosmic::executor::Default;

    type Flags = Flags;

    type Message = Message;

    const APP_ID: &'static str = "dev.edfloreshz.CosmicTweaks";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn header_start(&self) -> Vec<Element<Self::Message>> {
        let menu_bar = menu::bar(vec![menu::Tree::with_children(
            menu::root(fl!("view")),
            menu::items(
                &self.key_binds,
                vec![
                    menu::Item::Button(fl!("settings"), Action::Settings),
                    menu::Item::Button(fl!("about"), Action::About),
                ],
            ),
        )]);

        vec![menu_bar.into()]
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

    fn context_drawer(&self) -> Option<Element<Message>> {
        if !self.core.window.show_context {
            return None;
        }

        Some(match self.context_page {
            ContextPage::About => self.about(),
            ContextPage::Settings => self.settings(),
            ContextPage::AvailableThemes => self.available_themes(),
        })
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
            key_binds: key_binds(),
            color_schemes: ColorSchemes::default(),
            context_page: ContextPage::About,
            app_themes: vec![fl!("match-desktop"), fl!("dark"), fl!("light")],
            config_manager: ConfigManager::new(),
            available: vec![],
            status: Status::Idle,
            limit: 15,
            offset: 0,
        };

        let commands = vec![app.update(Message::FetchAvailableColorSchemes(
            ColorSchemeProvider::CosmicThemes,
            app.limit,
        ))];

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

    fn update(&mut self, message: Self::Message) -> cosmic::Command<app::Message<Self::Message>> {
        // Helper for updating config values efficiently
        macro_rules! config_set {
            ($name: ident, $value: expr) => {
                match &self.config_manager.app_handler {
                    Some(config_handler) => {
                        match paste::paste! { self.config_manager.app_config.[<set_ $name>](config_handler, $value) } {
                            Ok(_) => {}
                            Err(err) => {
                                log::warn!(
                                    "failed to save config {:?}: {}",
                                    stringify!($name),
                                    err
                                );
                            }
                        }
                    }
                    None => {
                        self.config_manager.app_config.$name = $value;
                        log::warn!(
                            "failed to save config {:?}: no config handler",
                            stringify!($name)
                        );
                    }
                }
            };
        }

        let mut commands = vec![];
        match message {
            Message::FetchAvailableColorSchemes(provider, limit) => {
                if self.offset == 0 {
                    self.status = Status::Loading;
                } else {
                    self.status = Status::LoadingMore;
                }
                self.limit = limit;
                self.offset = self.offset + self.limit;
                let limit = self.limit.clone();
                let offset = self.offset.clone();
                commands.push(Command::perform(
                    async move {
                        let url = match provider {
                            ColorSchemeProvider::CosmicThemes => {
                                format!("https://cosmic-themes.org/api/themes/?order=name&limit={}&offset={}", limit, offset)
                            }
                        };

                        let response = reqwest::get(url).await?;
                        let themes: Vec<CosmicTheme> = response.json().await?;
                        let available = themes
                            .into_iter()
                            .map(ColorScheme::from)
                            .collect();
                        Ok(available)
                    },
                    |res: Result<Vec<ColorScheme>, reqwest::Error>| match res {
                        Ok(themes) => cosmic::app::Message::App(Message::SetAvailableColorSchemes(themes)),
                        Err(e) => {
                            log::error!("{e}");
                            cosmic::app::Message::App(Message::SetAvailableColorSchemes(vec![]))
                        }
                    },
                ));
            }
            Message::SetAvailableColorSchemes(mut available) => {
                self.status = Status::Idle;
                self.available.append(&mut available);
            }
            Message::AppTheme(index) => {
                let app_theme = match index {
                    1 => AppTheme::Dark,
                    2 => AppTheme::Light,
                    _ => AppTheme::System,
                };
                config_set!(app_theme, app_theme);
                return self.update_config();
            }
            Message::LaunchUrl(url) => match open::that_detached(&url) {
                Ok(()) => {}
                Err(err) => {
                    log::warn!("failed to open {:?}: {}", url, err);
                }
            },
            Message::ToggleContextPage(page) => {
                if self.context_page == page {
                    self.core.window.show_context = !self.core.window.show_context;
                } else {
                    self.context_page = page.clone();
                    self.core.window.show_context = true;
                }
                self.set_context_title(page.clone().title());
            }
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
                pages::color_schemes::Message::OpenAvailableThemes => commands
                    .push(self.update(Message::ToggleContextPage(ContextPage::AvailableThemes))),
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

impl TweakTool {
    fn update_config(&mut self) -> Command<cosmic::app::Message<Message>> {
        app::command::set_theme(self.config_manager.app_config.app_theme.theme())
    }

    fn about(&self) -> Element<Message> {
        let spacing = cosmic::theme::active().cosmic().spacing;
        let repository = "https://github.com/cosmic-utils/tweaks";
        let hash = env!("VERGEN_GIT_SHA");
        let short_hash: String = hash.chars().take(7).collect();
        let date = env!("VERGEN_GIT_COMMIT_DATE");
        widget::column::with_children(vec![
            widget::svg(widget::svg::Handle::from_memory(
                &include_bytes!("../res/icons/hicolor/scalable/apps/icon.svg")[..],
            ))
            .into(),
            widget::text::title3(fl!("app-title")).into(),
            widget::button::link(repository)
                .on_press(Message::LaunchUrl(repository.to_string()))
                .padding(spacing.space_none)
                .into(),
            widget::button::link(fl!(
                "git-description",
                hash = short_hash.as_str(),
                date = date
            ))
            .on_press(Message::LaunchUrl(format!("{repository}/commits/{hash}")))
            .padding(spacing.space_none)
            .into(),
        ])
        .align_items(Alignment::Center)
        .spacing(spacing.space_xxs)
        .width(Length::Fill)
        .into()
    }

    fn settings(&self) -> Element<Message> {
        let app_theme_selected = match self.config_manager.app_config.app_theme {
            AppTheme::Dark => 1,
            AppTheme::Light => 2,
            AppTheme::System => 0,
        };
        widget::settings::view_column(vec![widget::settings::section()
            .title(fl!("appearance"))
            .add(
                widget::settings::item::builder(fl!("theme")).control(widget::dropdown(
                    &self.app_themes,
                    Some(app_theme_selected),
                    Message::AppTheme,
                )),
            )
            .into()])
        .into()
    }

    fn available_themes<'a>(&self) -> Element<'a, Message> {
        let spacing = cosmic::theme::active().cosmic().spacing;

        let loading: Option<Element<'a, Message>> = if let Status::Loading = self.status {
            Some(widget::text(fl!("loading")).into())
        } else {
            None
        };

        let available: Option<Element<'a, Message>> = match self.status {
            Status::Idle | Status::LoadingMore => Some(
                widget::settings::section()
                    .title(fl!("available"))
                    .add({
                        let themes: Vec<Element<Message>> =
                            self.available.iter().map(preview::available).collect();

                        widget::flex_row(themes)
                            .row_spacing(spacing.space_xs)
                            .column_spacing(spacing.space_xs)
                            .apply(widget::container)
                            .padding([0, spacing.space_xxs])
                    })
                    .into(),
            ),
            Status::Loading => None,
        };

        let show_more_button: Option<Element<'a, Message>> = match self.status {
            Status::Idle => Some(
                widget::button::text(fl!("show-more"))
                    .on_press(Message::FetchAvailableColorSchemes(
                        ColorSchemeProvider::CosmicThemes,
                        self.limit,
                    ))
                    .style(cosmic::theme::Button::Standard)
                    .into(),
            ),
            Status::LoadingMore => Some(
                widget::button::text(fl!("loading"))
                    .style(cosmic::theme::Button::Standard)
                    .into(),
            ),
            Status::Loading => None,
        };

        widget::settings::view_column(
            loading
                .into_iter()
                .chain(available.into_iter())
                .chain(show_more_button.into_iter())
                .collect(),
        )
        .into()
    }
}
