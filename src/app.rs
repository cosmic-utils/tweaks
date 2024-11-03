use std::{
    any::TypeId,
    collections::{HashMap, VecDeque},
};

use cosmic::{
    app::{self, Core},
    cosmic_config::{self, Update},
    cosmic_theme::{self, ThemeMode},
    iced::{
        event,
        keyboard::{Event as KeyEvent, Key, Modifiers},
        Alignment, Event, Length, Subscription,
    },
    widget::{
        self,
        menu::{self, Action, KeyBind},
        segmented_button,
    },
    Application, ApplicationExt, Apply, Element, Task,
};
use key_bind::key_binds;
use pages::color_schemes::providers::cosmic_themes::CosmicTheme;

use crate::{
    core::nav::NavPage,
    fl,
    pages::{
        self,
        color_schemes::{config::ColorScheme, preview, ColorSchemeProvider, ColorSchemes},
        layouts::Layouts,
    },
    settings::{AppTheme, TweaksSettings, CONFIG_VERSION},
};

mod key_bind;
pub mod style;

pub struct TweakTool {
    core: Core,
    nav_model: segmented_button::SingleSelectModel,
    dialog_pages: VecDeque<DialogPage>,
    dialog_text_input: widget::Id,
    key_binds: HashMap<KeyBind, TweaksAction>,
    modifiers: Modifiers,
    color_schemes: ColorSchemes,
    layouts: Layouts,
    context_page: ContextPage,
    app_themes: Vec<String>,
    config_handler: Option<cosmic_config::Config>,
    config: TweaksSettings,
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
    SaveCurrentColorScheme(String),
    SaveCurrentLayout(String),
}

#[derive(Debug, Clone)]
pub enum Message {
    Dock(pages::dock::Message),
    Panel(pages::panel::Message),
    Layouts(pages::layouts::Message),
    ColorSchemes(Box<pages::color_schemes::Message>),
    OpenSaveCurrentColorScheme,
    OpenSaveCurrentLayout,
    DialogUpdate(DialogPage),
    DialogComplete,
    DialogCancel,
    SaveNewColorScheme(String),
    ToggleContextPage(ContextPage),
    LaunchUrl(String),
    AppTheme(usize),
    FetchAvailableColorSchemes(ColorSchemeProvider, usize),
    SetAvailableColorSchemes(Vec<ColorScheme>),
    Key(Modifiers, Key),
    Modifiers(Modifiers),
    SystemThemeModeChange,
    SaveNewLayout(String),
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
pub enum TweaksAction {
    About,
    Settings,
}

impl Action for TweaksAction {
    type Message = Message;
    fn message(&self) -> Self::Message {
        match self {
            TweaksAction::About => Message::ToggleContextPage(ContextPage::About),
            TweaksAction::Settings => Message::ToggleContextPage(ContextPage::Settings),
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
                    menu::Item::Button(fl!("settings"), TweaksAction::Settings),
                    menu::Item::Button(fl!("about"), TweaksAction::About),
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
    ) -> cosmic::iced::Task<app::Message<Self::Message>> {
        self.nav_model.activate(id);

        let Some(win_id) = self.core.main_window_id() else {
            return Task::none();
        };

        let title = if let Some(page) = self.nav_model.data::<NavPage>(id) {
            format!("{} - {}", page.title(), fl!("app-title"))
        } else {
            fl!("app-title")
        };

        Task::batch(vec![self.set_window_title(title, win_id)])
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

        let text_input = match dialog_page {
            DialogPage::SaveCurrentColorScheme(name) => widget::text_input("", name.as_str())
                .id(self.dialog_text_input.clone())
                .on_input(move |name| {
                    Message::DialogUpdate(DialogPage::SaveCurrentColorScheme(name))
                }),
            DialogPage::SaveCurrentLayout(name) => widget::text_input("", name.as_str())
                .id(self.dialog_text_input.clone())
                .on_input(move |name| Message::DialogUpdate(DialogPage::SaveCurrentLayout(name))),
        };

        let dialog = widget::dialog(fl!("save-current-color-scheme"))
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
                    text_input.into(),
                ])
                .spacing(spacing.space_xxs),
            );

        Some(dialog.into())
    }

    fn init(core: Core, flags: Self::Flags) -> (Self, Task<app::Message<Self::Message>>) {
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
            modifiers: Modifiers::empty(),
            color_schemes: ColorSchemes::default(),
            layouts: Layouts::default(),
            context_page: ContextPage::About,
            app_themes: vec![fl!("match-desktop"), fl!("dark"), fl!("light")],
            config_handler: flags.config_handler,
            config: flags.config,
            available: vec![],
            status: Status::Idle,
            limit: 15,
            offset: 0,
        };

        let mut tasks = vec![app.update(Message::FetchAvailableColorSchemes(
            ColorSchemeProvider::CosmicThemes,
            app.limit,
        ))];

        if let Some(id) = app.core.main_window_id() {
            tasks.push(app.set_window_title(fl!("app-title"), id));
        }

        (app, Task::batch(tasks))
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
            NavPage::Layouts => self.layouts.view().map(Message::Layouts),
        };

        widget::column::with_children(vec![view])
            .padding(spacing.space_xs)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .into()
    }

    fn update(&mut self, message: Self::Message) -> cosmic::Task<app::Message<Self::Message>> {
        // Helper for updating config values efficiently
        macro_rules! config_set {
            ($name: ident, $value: expr) => {
                match &self.config_handler {
                    Some(config_handler) => {
                        match paste::paste! { self.config.[<set_ $name>](config_handler, $value) } {
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
                        self.config.$name = $value;
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
                self.offset += self.limit;
                let limit = self.limit;
                let offset = self.offset;
                commands.push(Task::perform(
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
            Message::Layouts(message) => match message {
                pages::layouts::Message::OpenSaveDialog => {
                    commands.push(self.update(Message::OpenSaveCurrentLayout))
                }
                _ => commands.push(self.layouts.update(message).map(cosmic::app::Message::App)),
            },
            Message::ColorSchemes(message) => match *message {
                pages::color_schemes::Message::SaveCurrentColorScheme(None) => {
                    commands.push(self.update(Message::OpenSaveCurrentColorScheme))
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
            Message::SaveNewLayout(name) => commands.push(self.update(Message::Layouts(
                pages::layouts::Message::SaveCurrentLayout(name),
            ))),
            Message::OpenSaveCurrentColorScheme => {
                self.dialog_pages
                    .push_back(DialogPage::SaveCurrentColorScheme(String::new()));
                return widget::text_input::focus(self.dialog_text_input.clone());
            }
            Message::OpenSaveCurrentLayout => {
                self.dialog_pages
                    .push_back(DialogPage::SaveCurrentLayout(String::new()));
                return widget::text_input::focus(self.dialog_text_input.clone());
            }
            Message::DialogUpdate(dialog_page) => {
                self.dialog_pages[0] = dialog_page;
            }
            Message::DialogComplete => {
                if let Some(dialog_page) = self.dialog_pages.pop_front() {
                    match dialog_page {
                        DialogPage::SaveCurrentColorScheme(name) => {
                            commands.push(self.update(Message::SaveNewColorScheme(name)))
                        }
                        DialogPage::SaveCurrentLayout(name) => {
                            commands.push(self.update(Message::SaveNewLayout(name)))
                        }
                    }
                }
            }
            Message::DialogCancel => {
                self.dialog_pages.pop_front();
            }
            Message::Key(modifiers, key) => {
                for (key_bind, action) in &self.key_binds {
                    if key_bind.matches(modifiers, &key) {
                        return self.update(action.message());
                    }
                }
            }
            Message::Modifiers(modifiers) => {
                self.modifiers = modifiers;
            }
            Message::SystemThemeModeChange => {
                commands.push(self.update_config());
            }
        }
        Task::batch(commands)
    }

    fn subscription(&self) -> cosmic::iced::Subscription<Self::Message> {
        struct ConfigSubscription;
        struct ThemeSubscription;

        let subscriptions = vec![
            event::listen_with(|event, _status, _window_id| match event {
                Event::Keyboard(KeyEvent::KeyPressed { key, modifiers, .. }) => {
                    Some(Message::Key(modifiers, key))
                }
                Event::Keyboard(KeyEvent::ModifiersChanged(modifiers)) => {
                    Some(Message::Modifiers(modifiers))
                }
                _ => None,
            }),
            cosmic_config::config_subscription(
                TypeId::of::<ConfigSubscription>(),
                Self::APP_ID.into(),
                CONFIG_VERSION,
            )
            .map(|update: Update<ThemeMode>| {
                if !update.errors.is_empty() {
                    log::info!(
                        "errors loading config {:?}: {:?}",
                        update.keys,
                        update.errors
                    );
                }
                Message::SystemThemeModeChange
            }),
            cosmic_config::config_subscription::<_, cosmic_theme::ThemeMode>(
                TypeId::of::<ThemeSubscription>(),
                cosmic_theme::THEME_MODE_ID.into(),
                cosmic_theme::ThemeMode::version(),
            )
            .map(|update: Update<ThemeMode>| {
                if !update.errors.is_empty() {
                    log::info!(
                        "errors loading theme mode {:?}: {:?}",
                        update.keys,
                        update.errors
                    );
                }
                Message::SystemThemeModeChange
            }),
        ];

        Subscription::batch(subscriptions)
    }
}

impl TweakTool {
    fn update_config(&mut self) -> Task<cosmic::app::Message<Message>> {
        app::command::set_theme(self.config.app_theme.theme())
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
        .align_x(Alignment::Center)
        .spacing(spacing.space_xxs)
        .width(Length::Fill)
        .into()
    }

    fn settings(&self) -> Element<Message> {
        let app_theme_selected = match self.config.app_theme {
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
                    .class(cosmic::style::Button::Standard)
                    .into(),
            ),
            Status::LoadingMore => Some(
                widget::button::text(fl!("loading"))
                    .class(cosmic::style::Button::Standard)
                    .into(),
            ),
            Status::Loading => None,
        };

        widget::settings::view_column(
            loading
                .into_iter()
                .chain(available)
                .chain(show_more_button)
                .collect(),
        )
        .into()
    }
}
