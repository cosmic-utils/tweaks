use cosmic::{iced::Alignment, widget, Apply, Element};

use crate::app::message::Message;
use crate::app::App;

use crate::fl;
use crate::app::core::icons;
use crate::app::pages::layouts::preview::{LayoutPreview, PanelProperties};
use super::Cosmic;

#[derive(Clone, Debug)]
pub enum DialogPage {
    SaveCurrentColorScheme(String),
    CreateSnapshot(String),
    CreateLayout(String, LayoutPreview, Option<String>),
}

impl Cosmic {
    pub fn dialog(app: &App) -> Option<Element<Message>> {
        let spacing = cosmic::theme::spacing();
        let dialog_page = match app.cosmic.dialog_pages.front() {
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
                    widget::button::suggested(fl!("create"))
                        .on_press_maybe(Some(Message::DialogComplete)),
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
            DialogPage::CreateLayout(name, preview, error) => {
                let preview_view = preview.view::<Message>(&spacing, 130);

                let name_input = widget::text_input(fl!("layout-name"), name)
                    .id(app.cosmic.dialog_text_input.clone())
                    .on_input(move |name| {
                        Message::DialogUpdate(DialogPage::CreateLayout(
                            name.clone(),
                            preview.clone(),
                            error.clone(),
                        ))
                    })
                    .on_submit(|_| Message::DialogComplete);

                let panel_section = widget::settings::section()
                    .title(fl!("panel"))
                    .add(
                        widget::settings::item::builder(fl!("show"))
                            .icon(icons::get_icon("resize-mode-symbolic", 18))
                            .control(widget::toggler(!preview.panel.hidden).on_toggle(|hidden| {
                                Message::DialogUpdate(DialogPage::CreateLayout(
                                    name.clone(),
                                    LayoutPreview {
                                        panel: PanelProperties {
                                            hidden: !hidden,
                                            ..preview.panel.clone()
                                        },
                                        ..preview.clone()
                                    },
                                    error.clone(),
                                ))
                            })),
                    )
                    .add(
                        widget::settings::item::builder(fl!("extend"))
                            .icon(icons::get_icon("resize-mode-symbolic", 18))
                            .control(widget::toggler(preview.panel.extend).on_toggle(|extend| {
                                Message::DialogUpdate(DialogPage::CreateLayout(
                                    name.clone(),
                                    LayoutPreview {
                                        panel: PanelProperties {
                                            extend,
                                            ..preview.panel.clone()
                                        },
                                        ..preview.clone()
                                    },
                                    error.clone(),
                                ))
                            })),
                    )
                    .add(
                        widget::settings::item::builder(fl!("position"))
                            .icon(icons::get_icon("resize-mode-symbolic", 18))
                            .control({
                                let name = name.clone();
                                let preview = preview.clone();
                                widget::segmented_button::horizontal(&app.layouts.panel_model)
                                    .on_activate(move |entity| {
                                        Message::UpdatePanelLayoutPosition(
                                            entity,
                                            name.clone(),
                                            preview.clone(),
                                        )
                                    })
                                    .button_alignment(Alignment::Center)
                                    .button_spacing(spacing.space_xxs)
                            }),
                    )
                    .add(
                        widget::settings::item::builder(fl!("size"))
                            .icon(icons::get_icon("resize-mode-symbolic", 18))
                            .control({
                                let name = name.clone();
                                let preview = preview.clone();
                                let error = error.clone();
                                widget::spin_button(
                                    preview.panel.size.to_string(),
                                    preview.panel.size,
                                    1.0,
                                    0.0,
                                    50.0,
                                    move |size| {
                                        Message::DialogUpdate(DialogPage::CreateLayout(
                                            name.clone(),
                                            LayoutPreview {
                                                panel: PanelProperties {
                                                    size,
                                                    ..preview.panel.clone()
                                                },
                                                ..preview.clone()
                                            },
                                            error.clone(),
                                        ))
                                    },
                                )
                            }),
                    );

                let dock_section = widget::settings::section()
                    .title(fl!("dock"))
                    .add(
                        widget::settings::item::builder(fl!("show"))
                            .icon(icons::get_icon("resize-mode-symbolic", 18))
                            .control(widget::toggler(!preview.dock.hidden).on_toggle(|hidden| {
                                Message::DialogUpdate(DialogPage::CreateLayout(
                                    name.clone(),
                                    LayoutPreview {
                                        dock: PanelProperties {
                                            hidden: !hidden,
                                            ..preview.dock.clone()
                                        },
                                        ..preview.clone()
                                    },
                                    error.clone(),
                                ))
                            })),
                    )
                    .add(
                        widget::settings::item::builder(fl!("extend"))
                            .icon(icons::get_icon("resize-mode-symbolic", 18))
                            .control(widget::toggler(preview.dock.extend).on_toggle(|extend| {
                                Message::DialogUpdate(DialogPage::CreateLayout(
                                    name.clone(),
                                    LayoutPreview {
                                        dock: PanelProperties {
                                            extend,
                                            ..preview.dock.clone()
                                        },
                                        ..preview.clone()
                                    },
                                    error.clone(),
                                ))
                            })),
                    )
                    .add(
                        widget::settings::item::builder(fl!("position"))
                            .icon(icons::get_icon("resize-mode-symbolic", 18))
                            .control({
                                let name = name.clone();
                                let preview = preview.clone();
                                widget::segmented_button::horizontal(&app.layouts.dock_model)
                                    .on_activate(move |entity| {
                                        Message::UpdateDockLayoutPosition(
                                            entity,
                                            name.clone(),
                                            preview.clone(),
                                        )
                                    })
                                    .button_alignment(Alignment::Center)
                                    .button_spacing(spacing.space_xxs)
                            }),
                    )
                    .add(
                        widget::settings::item::builder(fl!("size"))
                            .icon(icons::get_icon("resize-mode-symbolic", 18))
                            .control({
                                let name = name.clone();
                                let preview = preview.clone();
                                let error = error.clone();
                                widget::spin_button(
                                    preview.dock.size.to_string(),
                                    preview.dock.size,
                                    1.0,
                                    0.0,
                                    50.0,
                                    move |size| {
                                        Message::DialogUpdate(DialogPage::CreateLayout(
                                            name.clone(),
                                            LayoutPreview {
                                                dock: PanelProperties {
                                                    size,
                                                    ..preview.dock.clone()
                                                },
                                                ..preview.clone()
                                            },
                                            error.clone(),
                                        ))
                                    },
                                )
                            }),
                    )
                    .add(
                        widget::settings::item::builder(fl!("dock-icons"))
                            .icon(icons::get_icon("resize-mode-symbolic", 18))
                            .control({
                                let name = name.clone();
                                let preview = preview.clone();
                                let error = error.clone();
                                widget::spin_button(
                                    preview.dock_icons.to_string(),
                                    preview.dock_icons,
                                    1,
                                    1,
                                    20,
                                    move |size| {
                                        Message::DialogUpdate(DialogPage::CreateLayout(
                                            name.clone(),
                                            LayoutPreview {
                                                dock_icons: size,
                                                ..preview.clone()
                                            },
                                            error.clone(),
                                        ))
                                    },
                                )
                            }),
                    );

                widget::dialog()
                    .title(fl!("save-current-layout"))
                    .body(fl!("save-current-layout-description"))
                    .primary_action(widget::button::suggested(fl!("create")).on_press(
                        if name.is_empty() {
                            Message::DialogUpdate(DialogPage::CreateLayout(
                                name.clone(),
                                preview.clone(),
                                Some(fl!("layout-name-empty")),
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
                                    .push_maybe(if let Some(error) = error {
                                        Some(
                                            widget::text::caption(error.to_string())
                                                .class(cosmic::style::Text::Accent),
                                        )
                                    } else {
                                        None
                                    })
                                    .push(panel_section)
                                    .push(dock_section)
                                    .padding(spacing.space_s)
                                    .spacing(spacing.space_m)
                                    .apply(widget::scrollable)
                                    .height(300),
                            )
                            .spacing(spacing.space_m),
                    )
            }
        };

        Some(dialog.into())
    }
}
