use cosmic::{Element, widget};

use crate::app::App;
use crate::app::message::Message;
use crate::app::pages::layouts::dialog::{CreateLayoutDialog, PanelType};

use super::Cosmic;
use crate::fl;

#[derive(Clone, Debug)]
pub enum DialogPage {
    SaveCurrentColorScheme(String),
    CreateSnapshot(String),
    CreateLayout(CreateLayoutDialog),
}

impl Cosmic {
    pub fn dialog<'a>(app: &'a App) -> Option<Element<'a, Message>> {
        let spacing = cosmic::theme::spacing();
        let dialog_page = app.cosmic.dialog_pages.front()?;

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
                                .id(app.cosmic.dialog_text_input.clone())
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
                    widget::button::standard(fl!("create")).on_press(Message::DialogComplete),
                )
                .secondary_action(
                    widget::button::standard(fl!("cancel")).on_press(Message::DialogCancel),
                )
                .control(
                    widget::text_input(fl!("snapshot-name"), name.as_str())
                        .id(app.cosmic.dialog_text_input.clone())
                        .on_input(move |name| {
                            Message::DialogUpdate(DialogPage::CreateSnapshot(name))
                        })
                        .on_submit(|_| Message::DialogComplete),
                ),
            DialogPage::CreateLayout(dialog) => {
                let CreateLayoutDialog {
                    name,
                    preview,
                    error,
                } = dialog;
                let preview_view = preview.view::<Message>(&spacing, 130);

                let name_input =
                    widget::text_input(fl!("layout-name"), name)
                        .id(app.cosmic.dialog_text_input.clone())
                        .on_input(move |name| {
                            Message::DialogUpdate(DialogPage::CreateLayout(
                                CreateLayoutDialog::new(name.clone(), *preview, error.clone()),
                            ))
                        })
                        .on_submit(|_| Message::DialogComplete);

                widget::dialog()
                    .width(700)
                    .title(fl!("save-current-layout"))
                    .body(fl!("save-current-layout-description"))
                    .primary_action(widget::button::suggested(fl!("create")).on_press(
                        if name.is_empty() {
                            Message::DialogUpdate(DialogPage::CreateLayout(
                                CreateLayoutDialog::new(
                                    name.clone(),
                                    *preview,
                                    Some(fl!("layout-name-empty")),
                                ),
                            ))
                        } else {
                            Message::DialogComplete
                        },
                    ))
                    .secondary_action(
                        widget::button::standard(fl!("cancel")).on_press(Message::DialogCancel),
                    )
                    .control(
                        widget::column()
                            .push(preview_view)
                            .push(
                                widget::column()
                                    .push(name_input)
                                    .push_maybe(error.as_ref().map(|error| {
                                        widget::text::caption(error.to_string())
                                            .class(cosmic::style::Text::Accent)
                                    }))
                                    .push(
                                        widget::scrollable(
                                            widget::column()
                                                .push(dialog.section(
                                                    PanelType::Panel,
                                                    &app.layouts.panel_model,
                                                ))
                                                .push(dialog.section(
                                                    PanelType::Dock,
                                                    &app.layouts.dock_model,
                                                )),
                                        )
                                        .height(300),
                                    )
                                    .padding(spacing.space_s)
                                    .spacing(spacing.space_m),
                            )
                            .spacing(spacing.space_m),
                    )
            }
        };

        Some(dialog.into())
    }
}
