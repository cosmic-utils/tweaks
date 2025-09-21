use cosmic::{
    Application, Task,
    app::{self},
    widget::{self, menu::Action},
};

use crate::app::{App, dialog::DialogPage, pages::snapshots::config::SnapshotKind};
use crate::app::{message::Message, pages::layouts::dialog::CreateLayoutDialog};

use super::Cosmic;
use crate::app::core::config::AppTheme;
use crate::app::pages::{self, layouts::preview::Position};

impl Cosmic {
    pub fn update(app: &mut App, message: Message) -> app::Task<Message> {
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
                if let Err(err) = app.config.set_app_theme(&app.handler, app_theme) {
                    log::warn!("failed to save config: {}", err);
                };
                tasks.push(app.update_config());
            }
            Message::ToggleContextPage(page) => {
                if app.cosmic.context_page == page {
                    app.core_mut().window.show_context = !app.core().window.show_context;
                } else {
                    app.cosmic.context_page = page;
                    app.core_mut().window.show_context = true;
                }
            }
            Message::ToggleContextDrawer => {
                app.core_mut().window.show_context = !app.core().window.show_context;
            }
            Message::Dock(message) => tasks.push(app.dock.update(message).map(cosmic::action::app)),
            Message::Panel(message) => {
                tasks.push(app.panel.update(message).map(cosmic::action::app))
            }
            Message::Layouts(message) => {
                tasks.push(app.layouts.update(message).map(cosmic::action::app))
            }
            Message::Shortcuts(message) => {
                tasks.push(app.shortcuts.update(message).map(cosmic::action::app))
            }
            Message::Snapshots(message) => {
                tasks.push(app.snapshots.update(message).map(cosmic::action::app))
            }
            Message::ColorSchemes(message) => match *message {
                pages::color_schemes::Message::SaveCurrentColorScheme(None) => {
                    tasks.push(app.update(Message::ToggleDialogPage(
                        DialogPage::SaveCurrentColorScheme(String::new()),
                    )))
                }
                _ => tasks.push(
                    app.color_schemes
                        .update(*message)
                        .map(Box::new)
                        .map(Message::ColorSchemes)
                        .map(cosmic::action::app),
                ),
            },
            Message::UpdatePanelLayoutPosition(entity, name, mut preview) => {
                app.layouts.panel_model.activate(entity);
                if let Some(position) = app.layouts.panel_model.data::<Position>(entity) {
                    preview.panel.position = *position;
                    tasks.push(app.update(Message::DialogUpdate(DialogPage::CreateLayout(
                        CreateLayoutDialog::new(name, preview, None),
                    ))))
                }
            }
            Message::UpdateDockLayoutPosition(entity, name, mut preview) => {
                app.layouts.dock_model.activate(entity);
                if let Some(position) = app.layouts.dock_model.data::<Position>(entity) {
                    preview.dock.position = *position;
                    tasks.push(app.update(Message::DialogUpdate(DialogPage::CreateLayout(
                        CreateLayoutDialog::new(name, preview, None),
                    ))))
                }
            }
            Message::SaveNewColorScheme(name) => {
                tasks.push(app.update(Message::ColorSchemes(Box::new(
                    pages::color_schemes::Message::SaveCurrentColorScheme(Some(name)),
                ))))
            }
            Message::ToggleDialogPage(dialog_page) => {
                app.cosmic.dialog_pages.push_back(dialog_page);
                tasks.push(widget::text_input::focus(
                    app.cosmic.dialog_text_input.clone(),
                ));
            }
            Message::DialogUpdate(dialog_page) => {
                app.cosmic.dialog_pages[0] = dialog_page;
            }
            Message::DialogComplete => {
                if let Some(dialog_page) = app.cosmic.dialog_pages.pop_front() {
                    match dialog_page {
                        DialogPage::SaveCurrentColorScheme(name) => {
                            tasks.push(app.update(Message::SaveNewColorScheme(name)))
                        }
                        DialogPage::CreateSnapshot(name) => {
                            tasks.push(app.update(Message::Snapshots(
                                pages::snapshots::Message::CreateSnapshot(name, SnapshotKind::User),
                            )))
                        }
                        DialogPage::CreateLayout(dialog) => {
                            let CreateLayoutDialog {
                                name,
                                preview,
                                error,
                            } = dialog;
                            if let Some(error) = error {
                                tasks.push(app.update(Message::ToggleDialogPage(
                                    DialogPage::CreateLayout(CreateLayoutDialog::new(
                                        name,
                                        preview,
                                        Some(error),
                                    )),
                                )));
                            } else {
                                tasks.push(app.update(Message::Layouts(
                                    pages::layouts::Message::Create(name, preview),
                                )));
                            }
                        }
                    }
                }
            }
            Message::DialogCancel => {
                app.cosmic.dialog_pages.pop_front();
            }
            Message::Key(modifiers, key) => {
                for (key_bind, action) in &app.cosmic.key_binds {
                    if key_bind.matches(modifiers, &key) {
                        return app.update(action.message());
                    }
                }
            }
            Message::Modifiers(modifiers) => {
                app.cosmic.modifiers = modifiers;
            }
            Message::SystemThemeModeChange => {
                tasks.push(app.update_config());
            }
        }
        Task::batch(tasks)
    }
}
