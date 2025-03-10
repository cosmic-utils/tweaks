use std::{
    any::TypeId,
    collections::{HashMap, VecDeque},
};

use cosmic::{
    app::{self, context_drawer::ContextDrawer, Core},
    cosmic_config::{self, Update},
    cosmic_theme::{self, ThemeMode},
    iced::{
        event,
        keyboard::{Event as KeyEvent, Key, Modifiers},
        Alignment, Event, Length, Subscription,
    },
    widget::{
        self,
        about::About,
        menu::{self, Action, ItemHeight, ItemWidth, KeyBind},
        segmented_button,
    },
    Application, ApplicationExt, Apply, Element, Task,
};
use key_bind::key_binds;

use crate::{
    app::{
        config::{AppTheme, TweaksConfig, CONFIG_VERSION},
        nav::Page,
    },
    core::icons,
    fl,
    pages::{
        self,
        color_schemes::{self, ColorSchemes, Tab},
        dock::Dock,
        layouts::Layouts,
        panel::Panel,
        shortcuts::Shortcuts,
        snapshots::{config::SnapshotKind, Snapshots},
    },
};

pub mod config;
pub mod cosmic_panel_button_config;
mod key_bind;
pub mod nav;
pub mod resources;
pub mod settings;
pub mod style;

pub struct TweakTool {
    core: Core,
    nav_model: segmented_button::SingleSelectModel,
    about: About,
    dialog_pages: VecDeque<DialogPage>,
    dialog_text_input: widget::Id,
    key_binds: HashMap<KeyBind, TweaksAction>,
    modifiers: Modifiers,
    color_schemes: ColorSchemes,
    dock: Dock,
    panel: Panel,
    layouts: Layouts,
    snapshots: Snapshots,
    shorcuts: Shortcuts,
    context_page: ContextPage,
    app_themes: Vec<String>,
    config_handler: Option<cosmic_config::Config>,
    config: TweaksConfig,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DialogPage {
    SaveCurrentColorScheme(String),
    CreateSnapshot(String),
}

#[derive(Debug, Clone)]
pub enum Message {
    Dock(pages::dock::Message),
    Panel(pages::panel::Message),
    Layouts(pages::layouts::Message),
    Shortcuts(pages::shortcuts::Message),
    Snapshots(pages::snapshots::Message),
    ColorSchemes(Box<pages::color_schemes::Message>),
    DialogUpdate(DialogPage),
    DialogComplete,
    DialogCancel,
    SaveNewColorScheme(String),
    ToggleContextPage(ContextPage),
    ToggleContextDrawer,
    ToggleDialogPage(DialogPage),
    AppTheme(usize),
    Key(Modifiers, Key),
    Modifiers(Modifiers),
    SystemThemeModeChange,
    Open(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ContextPage {
    Settings,
    About,
}

impl ContextPage {
    fn title(&self) -> String {
        match self {
            Self::About => fl!("about"),
            Self::Settings => fl!("settings"),
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
    pub config: TweaksConfig,
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

    fn init(core: Core, flags: Self::Flags) -> (Self, Task<app::Message<Self::Message>>) {
        log::info!("Starting Cosmic Tweak Tool...");

        let mut nav_model = segmented_button::SingleSelectModel::default();
        for &nav_page in Page::all() {
            let id = nav_model
                .insert()
                .icon(nav_page.icon())
                .text(nav_page.title())
                .data::<Page>(nav_page)
                .id();

            if nav_page == Page::default() {
                nav_model.activate(id);
            }
        }

        let about = About::default()
            .name(fl!("app-title"))
            .icon(Self::APP_ID)
            .version("0.1.3")
            .author("Eduardo Flores")
            .license("GPL-3.0-only")
            .links([
                (
                    fl!("support"),
                    "https://github.com/cosmic-utils/tweaks/issues",
                ),
                (fl!("repository"), "https://github.com/cosmic-utils/tweaks"),
            ])
            .developers([("Eduardo Flores", "edfloreshz@proton.me")]);

        let mut app = TweakTool {
            core,
            nav_model,
            about,
            dialog_pages: VecDeque::new(),
            dialog_text_input: widget::Id::unique(),
            key_binds: key_binds(),
            modifiers: Modifiers::empty(),
            color_schemes: ColorSchemes::default(),
            layouts: Layouts::default(),
            dock: Dock::default(),
            panel: Panel::default(),
            snapshots: Snapshots::default(),
            context_page: ContextPage::About,
            app_themes: vec![fl!("match-desktop"), fl!("dark"), fl!("light")],
            config_handler: flags.config_handler,
            config: flags.config,
            shorcuts: Shortcuts::new(),
        };

        let mut tasks = vec![
            app.update(Message::ColorSchemes(Box::new(
                color_schemes::Message::FetchAvailableColorSchemes(
                    color_schemes::ColorSchemeProvider::CosmicThemes,
                    app.color_schemes.limit,
                ),
            ))),
            app.update(Message::Snapshots(
                pages::snapshots::Message::CreateSnapshot(
                    fl!("application-opened"),
                    SnapshotKind::System,
                ),
            )),
        ];

        tasks.push(app.set_window_title(fl!("app-title")));

        (app, Task::batch(tasks))
    }

    fn header_start(&self) -> Vec<Element<Self::Message>> {
        let menu_bar = menu::bar(vec![menu::Tree::with_children(
            menu::root(fl!("view")),
            menu::items(
                &self.key_binds,
                vec![
                    menu::Item::Button(
                        fl!("settings"),
                        Some(icons::get_handle("settings-symbolic", 14)),
                        TweaksAction::Settings,
                    ),
                    menu::Item::Divider,
                    menu::Item::Button(
                        fl!("about"),
                        Some(icons::get_handle("info-outline-symbolic", 14)),
                        TweaksAction::About,
                    ),
                ],
            ),
        )])
        .item_height(ItemHeight::Dynamic(40))
        .item_width(ItemWidth::Uniform(240))
        .spacing(4.0);

        vec![menu_bar.into()]
    }

    fn nav_model(&self) -> Option<&widget::nav_bar::Model> {
        Some(&self.nav_model)
    }

    fn on_nav_select(
        &mut self,
        id: widget::nav_bar::Id,
    ) -> cosmic::iced::Task<app::Message<Self::Message>> {
        self.nav_model.activate(id);

        let title = if let Some(page) = self.nav_model.data::<Page>(id) {
            format!("{} - {}", page.title(), fl!("app-title"))
        } else {
            fl!("app-title")
        };

        Task::batch(vec![self.set_window_title(title)])
    }

    fn context_drawer(&self) -> Option<ContextDrawer<Self::Message>> {
        if !self.core.window.show_context {
            return None;
        }

        Some(match self.context_page {
            ContextPage::About => {
                app::context_drawer::about(&self.about, Message::Open, Message::ToggleContextDrawer)
            }
            ContextPage::Settings => {
                app::context_drawer::context_drawer(self.settings(), Message::ToggleContextDrawer)
                    .title(self.context_page.title())
            }
        })
    }

    fn dialog(&self) -> Option<Element<Self::Message>> {
        let dialog_page = match self.dialog_pages.front() {
            Some(some) => some,
            None => return None,
        };

        let spacing = cosmic::theme::active().cosmic().spacing;

        let dialog = match dialog_page {
            DialogPage::SaveCurrentColorScheme(name) => widget::dialog()
                .title(fl!("save-current-color-scheme"))
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
                            .on_input(move |name| {
                                Message::DialogUpdate(DialogPage::SaveCurrentColorScheme(name))
                            })
                            .on_submit(Message::DialogComplete)
                            .into(),
                    ])
                    .spacing(spacing.space_xxs),
                ),
            DialogPage::CreateSnapshot(name) => widget::dialog()
                .title(fl!("create-snapshot"))
                .body(fl!("create-snapshot-description"))
                .primary_action(
                    widget::button::suggested(fl!("create"))
                        .on_press_maybe(Some(Message::DialogComplete)),
                )
                .secondary_action(
                    widget::button::standard(fl!("cancel")).on_press(Message::DialogCancel),
                )
                .control(
                    widget::text_input(fl!("snapshot-name"), name.as_str())
                        .id(self.dialog_text_input.clone())
                        .on_input(move |name| {
                            Message::DialogUpdate(DialogPage::CreateSnapshot(name))
                        })
                        .on_submit(Message::DialogComplete),
                ),
        };

        Some(dialog.into())
    }

    fn view(&self) -> Element<Self::Message> {
        let spacing = cosmic::theme::active().cosmic().spacing;
        let entity = self.nav_model.active();
        let nav_page = self.nav_model.data::<Page>(entity).unwrap_or_default();

        let view = match nav_page {
            Page::ColorSchemes => self
                .color_schemes
                .view()
                .map(Box::new)
                .map(Message::ColorSchemes),
            Page::Dock => self.dock.view().map(Message::Dock),
            Page::Panel => self.panel.view().map(Message::Panel),
            Page::Layouts => self.layouts.view().map(Message::Layouts),
            Page::Snapshots => self.snapshots.view().map(Message::Snapshots),
            Page::Shortcuts => self.shorcuts.view().map(Message::Shortcuts),
        };

        widget::column::with_children(vec![view])
            .padding(spacing.space_xs)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .into()
    }

    fn footer(&self) -> Option<Element<Self::Message>> {
        let spacing = cosmic::theme::active().cosmic().spacing;
        if let Some(Tab::Installed) = self.color_schemes.model.active_data::<Tab>() {
            Some(
                widget::row()
                    .push(widget::horizontal_space())
                    .push(
                        widget::button::standard(fl!("save-current-color-scheme"))
                            .trailing_icon(icons::get_handle("arrow-into-box-symbolic", 16))
                            .on_press(Message::ColorSchemes(Box::new(
                                color_schemes::Message::SaveCurrentColorScheme(None),
                            ))),
                    )
                    .push(
                        widget::button::standard(fl!("import-color-scheme"))
                            .trailing_icon(icons::get_handle("document-save-symbolic", 16))
                            .on_press(Message::ColorSchemes(Box::new(
                                color_schemes::Message::StartImport,
                            ))),
                    )
                    .spacing(spacing.space_xxs)
                    .apply(widget::container)
                    .class(cosmic::style::Container::Card)
                    .padding(spacing.space_xxs)
                    .into(),
            )
        } else {
            None
        }
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

        let mut tasks = vec![];
        match message {
            Message::Open(url) => {
                if let Err(err) = open::that_detached(url) {
                    log::error!("{err}")
                }
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
            Message::ToggleContextPage(page) => {
                if self.context_page == page {
                    self.core.window.show_context = !self.core.window.show_context;
                } else {
                    self.context_page = page;
                    self.core.window.show_context = true;
                }
            }
            Message::ToggleContextDrawer => {
                self.core.window.show_context = !self.core.window.show_context;
            }
            Message::Dock(message) => {
                tasks.push(self.dock.update(message).map(cosmic::app::Message::App))
            }
            Message::Panel(message) => {
                tasks.push(self.panel.update(message).map(cosmic::app::Message::App))
            }
            Message::Layouts(message) => {
                tasks.push(self.layouts.update(message).map(cosmic::app::Message::App))
            }
            Message::Shortcuts(message) => {
                tasks.push(self.shorcuts.update(message).map(cosmic::app::Message::App))
            }
            Message::Snapshots(message) => match message {
                pages::snapshots::Message::OpenSaveDialog => tasks.push(self.update(
                    Message::ToggleDialogPage(DialogPage::CreateSnapshot(String::new())),
                )),
                _ => tasks.push(
                    self.snapshots
                        .update(message)
                        .map(cosmic::app::Message::App),
                ),
            },
            Message::ColorSchemes(message) => match *message {
                pages::color_schemes::Message::SaveCurrentColorScheme(None) => {
                    tasks.push(self.update(Message::ToggleDialogPage(
                        DialogPage::SaveCurrentColorScheme(String::new()),
                    )))
                }
                _ => tasks.push(
                    self.color_schemes
                        .update(*message)
                        .map(Box::new)
                        .map(Message::ColorSchemes)
                        .map(cosmic::app::Message::App),
                ),
            },
            Message::SaveNewColorScheme(name) => {
                tasks.push(self.update(Message::ColorSchemes(Box::new(
                    pages::color_schemes::Message::SaveCurrentColorScheme(Some(name)),
                ))))
            }
            Message::ToggleDialogPage(dialog_page) => {
                self.dialog_pages.push_back(dialog_page);
                tasks.push(widget::text_input::focus(self.dialog_text_input.clone()));
            }
            Message::DialogUpdate(dialog_page) => {
                self.dialog_pages[0] = dialog_page;
            }
            Message::DialogComplete => {
                if let Some(dialog_page) = self.dialog_pages.pop_front() {
                    match dialog_page {
                        DialogPage::SaveCurrentColorScheme(name) => {
                            tasks.push(self.update(Message::SaveNewColorScheme(name)))
                        }
                        DialogPage::CreateSnapshot(name) => {
                            tasks.push(self.update(Message::Snapshots(
                                pages::snapshots::Message::CreateSnapshot(name, SnapshotKind::User),
                            )))
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
                tasks.push(self.update_config());
            }
        }
        Task::batch(tasks)
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
}
