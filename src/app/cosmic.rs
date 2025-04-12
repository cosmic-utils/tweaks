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
        keyboard::{Event as KeyEvent, Modifiers},
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

use super::flags::Flags;
use super::message::Message;
use super::page::Page;
use super::{action::TweaksAction, context::ContextPage};
use super::{dialog::DialogPage, App};

use crate::{
    core::{
        config::{AppTheme, CONFIG_VERSION},
        icons,
        key_bindings::KeyBindings,
    },
    fl,
    pages::{
        self,
        color_schemes::{self, ColorSchemes, Status, Tab},
        dock::Dock,
        layouts::Layouts,
        panel::Panel,
        shortcuts::Shortcuts,
        snapshots::{config::SnapshotKind, Snapshots},
    },
};

pub struct Cosmic {
    core: Core,
    nav_model: segmented_button::SingleSelectModel,
    about: About,
    dialog_pages: VecDeque<DialogPage>,
    dialog_text_input: widget::Id,
    key_binds: HashMap<KeyBind, TweaksAction>,
    modifiers: Modifiers,
    context_page: ContextPage,
    pub app_themes: Vec<String>,
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

        let mut app = App {
            cosmic: Cosmic {
                core,
                nav_model,
                about,
                dialog_pages: VecDeque::new(),
                dialog_text_input: widget::Id::unique(),
                key_binds: KeyBindings::new(),
                modifiers: Modifiers::empty(),
                context_page: ContextPage::About,
                app_themes: vec![fl!("match-desktop"), fl!("dark"), fl!("light")],
            },
            handler: flags.handler,
            config: flags.config,
            color_schemes: ColorSchemes::default(),
            layouts: Layouts::default(),
            dock: Dock::default(),
            panel: Panel::default(),
            snapshots: Snapshots::default(),
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
                &self.cosmic.key_binds,
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
        Some(&self.cosmic.nav_model)
    }

    fn on_nav_select(&mut self, id: widget::nav_bar::Id) -> app::Task<Self::Message> {
        self.cosmic.nav_model.activate(id);

        let title = if let Some(page) = self.cosmic.nav_model.data::<Page>(id) {
            format!("{} - {}", page.title(), fl!("app-title"))
        } else {
            fl!("app-title")
        };

        Task::batch(vec![self.set_window_title(title)])
    }

    fn context_drawer(&self) -> Option<ContextDrawer<Self::Message>> {
        if !self.core().window.show_context {
            return None;
        }

        Some(match self.cosmic.context_page {
            ContextPage::About => app::context_drawer::about(
                &self.cosmic.about,
                Message::Open,
                Message::ToggleContextDrawer,
            ),
            ContextPage::Settings => {
                app::context_drawer::context_drawer(self.settings(), Message::ToggleContextDrawer)
                    .title(self.cosmic.context_page.title())
            }
        })
    }

    fn dialog(&self) -> Option<Element<Self::Message>> {
        let spacing = cosmic::theme::spacing();
        let dialog_page = match self.cosmic.dialog_pages.front() {
            Some(some) => some,
            None => return None,
        };

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
                    widget::column()
                        .push(widget::text::body(fl!("color-scheme-name")))
                        .push(
                            widget::text_input("", name.as_str())
                                .id(self.cosmic.dialog_text_input.clone())
                                .on_input(move |name| {
                                    Message::DialogUpdate(DialogPage::SaveCurrentColorScheme(name))
                                })
                                .on_submit(|_| Message::DialogComplete),
                        )
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
                        .id(self.cosmic.dialog_text_input.clone())
                        .on_input(move |name| {
                            Message::DialogUpdate(DialogPage::CreateSnapshot(name))
                        })
                        .on_submit(|_| Message::DialogComplete),
                ),
        };

        Some(dialog.into())
    }

    fn view(&self) -> Element<Self::Message> {
        let spacing = cosmic::theme::spacing();
        let entity = self.cosmic.nav_model.active();
        let nav_page = self
            .cosmic
            .nav_model
            .data::<Page>(entity)
            .unwrap_or_default();

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

        widget::column()
            .push(view)
            .padding(spacing.space_xs)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .into()
    }

    fn footer(&self) -> Option<Element<Self::Message>> {
        let spacing = cosmic::theme::spacing();

        match self.cosmic.nav_model.active_data::<Page>() {
            Some(Page::ColorSchemes) => match self.color_schemes.model.active_data::<Tab>() {
                Some(Tab::Installed) => Some(
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
                ),
                Some(Tab::Available) => Some(
                    widget::row()
                        .push(widget::horizontal_space())
                        .push(match self.color_schemes.status {
                            Status::Idle => widget::button::standard(fl!("show-more"))
                                .leading_icon(crate::core::icons::get_handle(
                                    "content-loading-symbolic",
                                    16,
                                ))
                                .on_press(Message::ColorSchemes(Box::new(
                                    color_schemes::Message::FetchAvailableColorSchemes(
                                        color_schemes::ColorSchemeProvider::CosmicThemes,
                                        self.color_schemes.limit,
                                    ),
                                ))),
                            Status::LoadingMore | Status::Loading => {
                                widget::button::standard(fl!("loading"))
                            }
                        })
                        .spacing(spacing.space_xxs)
                        .apply(widget::container)
                        .class(cosmic::style::Container::Card)
                        .padding(spacing.space_xxs)
                        .into(),
                ),
                None => None,
            },
            _ => None,
        }
    }

    fn update(&mut self, message: Self::Message) -> app::Task<Self::Message> {
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
                if let Err(err) = self.config.set_app_theme(&self.handler, app_theme) {
                    log::warn!("failed to save config: {}", err);
                };
                tasks.push(self.update_config());
            }
            Message::ToggleContextPage(page) => {
                if self.cosmic.context_page == page {
                    self.core_mut().window.show_context = !self.core().window.show_context;
                } else {
                    self.cosmic.context_page = page;
                    self.core_mut().window.show_context = true;
                }
            }
            Message::ToggleContextDrawer => {
                self.core_mut().window.show_context = !self.core().window.show_context;
            }
            Message::Dock(message) => {
                tasks.push(self.dock.update(message).map(cosmic::action::app))
            }
            Message::Panel(message) => {
                tasks.push(self.panel.update(message).map(cosmic::action::app))
            }
            Message::Layouts(message) => {
                tasks.push(self.layouts.update(message).map(cosmic::action::app))
            }
            Message::Shortcuts(message) => {
                tasks.push(self.shorcuts.update(message).map(cosmic::action::app))
            }
            Message::Snapshots(message) => match message {
                pages::snapshots::Message::OpenSaveDialog => tasks.push(self.update(
                    Message::ToggleDialogPage(DialogPage::CreateSnapshot(String::new())),
                )),
                _ => tasks.push(self.snapshots.update(message).map(cosmic::action::app)),
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
                        .map(cosmic::action::app),
                ),
            },
            Message::SaveNewColorScheme(name) => {
                tasks.push(self.update(Message::ColorSchemes(Box::new(
                    pages::color_schemes::Message::SaveCurrentColorScheme(Some(name)),
                ))))
            }
            Message::ToggleDialogPage(dialog_page) => {
                self.cosmic.dialog_pages.push_back(dialog_page);
                tasks.push(widget::text_input::focus(
                    self.cosmic.dialog_text_input.clone(),
                ));
            }
            Message::DialogUpdate(dialog_page) => {
                self.cosmic.dialog_pages[0] = dialog_page;
            }
            Message::DialogComplete => {
                if let Some(dialog_page) = self.cosmic.dialog_pages.pop_front() {
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
                self.cosmic.dialog_pages.pop_front();
            }
            Message::Key(modifiers, key) => {
                for (key_bind, action) in &self.cosmic.key_binds {
                    if key_bind.matches(modifiers, &key) {
                        return self.update(action.message());
                    }
                }
            }
            Message::Modifiers(modifiers) => {
                self.cosmic.modifiers = modifiers;
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
