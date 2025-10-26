use std::collections::VecDeque;

use cosmic::{
    Application, ApplicationExt, Task,
    app::{self, Core},
    iced::keyboard::Modifiers,
    widget::{self, about::About},
};

use crate::app::App;
use crate::app::flags::Flags;
use crate::app::message::Message;
use crate::app::page::Page;
use crate::app::{context::ContextPage, pages::snapshots::config::SnapshotKind};

use super::Cosmic;
use crate::app::core::key_bindings::KeyBindings;
use crate::app::pages::{
    self, color_schemes::ColorSchemes, dock::Dock, layouts::Layouts, panel::Panel,
    shortcuts::ShortcutsPage, snapshots::Snapshots,
};
use crate::fl;

impl Cosmic {
    pub fn init(core: Core, flags: Flags) -> (App, app::Task<Message>) {
        log::info!("Starting Cosmic Tweak Tool...");

        let mut nav_model = widget::segmented_button::SingleSelectModel::default();
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
            .icon(widget::icon::from_name(App::APP_ID))
            .version("0.2.1")
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

        let mut tasks = vec![];

        let (color_schemes, task) = ColorSchemes::new();

        tasks.push(task.map(|m| cosmic::Action::App(Message::ColorSchemes(Box::new(m)))));

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
            color_schemes,
            layouts: Layouts::default(),
            dock: Dock::default(),
            panel: Panel::default(),
            snapshots: Snapshots::default(),
            shortcuts: ShortcutsPage::new(),
        };

        tasks.push(app.update(Message::Snapshots(
            pages::snapshots::Message::CreateSnapshot(
                fl!("application-opened"),
                SnapshotKind::System,
            ),
        )));

        match pages::layouts::config::Layout::list() {
            Ok(list) => {
                tasks.push(app.update(Message::Layouts(pages::layouts::Message::LoadLayouts(list))))
            }
            Err(error) => log::error!("Failed to load layouts: {}", error),
        }

        tasks.push(app.set_window_title(fl!("app-title")));

        (app, Task::batch(tasks))
    }
}
