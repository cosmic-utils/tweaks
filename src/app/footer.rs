use cosmic::{
    Apply, Element,
    widget::{self},
};

use crate::app::{message::Message, pages::layouts::dialog::CreateLayoutDialog};
use crate::app::{page::Page, pages};
use crate::{
    app::{App, dialog::DialogPage},
    icon_handle,
};

use super::Cosmic;
use crate::app::pages::layouts::preview::LayoutPreview;

impl Cosmic {
    pub fn footer<'a>(app: &'a App) -> Option<Element<'a, Message>> {
        let spacing = cosmic::theme::spacing();

        match app.cosmic.nav_model.active_data::<Page>()? {
            Page::ColorSchemes => {
                Some(
                    widget::row(vec![])
                        .push_maybe(app.color_schemes.footer().map(|elem| {
                            elem.map(|message| Message::ColorSchemes(Box::new(message)))
                        }))
                        .push(widget::space::horizontal())
                        .push(
                            widget::button::destructive(fl!("reset-to-defaults"))
                                .trailing_icon(icon_handle!("edit-undo-symbolic", 16))
                                .spacing(spacing.space_xs)
                                .on_press(Message::ToggleDialogPage(DialogPage::ResetPage(
                                    Page::ColorSchemes,
                                ))),
                        )
                        .spacing(spacing.space_xxs)
                        .apply(widget::container)
                        .class(cosmic::style::Container::Card)
                        .padding(spacing.space_xxs)
                        .into(),
                )
            }
            Page::Layouts => Some(
                widget::row(vec![])
                    .push(
                        widget::button::standard(fl!("save-current-layout"))
                            .trailing_icon(icon_handle!("arrow-into-box-symbolic", 16))
                            .spacing(spacing.space_xs)
                            .on_press(Message::ToggleDialogPage(DialogPage::CreateLayout(
                                CreateLayoutDialog::new(
                                    String::new(),
                                    LayoutPreview::default(),
                                    None,
                                ),
                            ))),
                    )
                    .push(widget::space::horizontal())
                    .push_maybe(app.layouts.selected_layout.as_ref().map(|_| {
                        widget::button::standard(fl!("apply-layout"))
                            .trailing_icon(icon_handle!("checkmark-symbolic", 16))
                            .spacing(spacing.space_xs)
                            .on_press(Message::Layouts(pages::layouts::Message::Apply))
                    }))
                    .push_maybe(app.layouts.selected_layout.as_ref().and_then(|selected| {
                        if selected.custom {
                            Some(
                                widget::button::standard(fl!("delete-layout"))
                                    .trailing_icon(icon_handle!("recycling-bin-symbolic", 16))
                                    .spacing(spacing.space_xs)
                                    .on_press(Message::Layouts(pages::layouts::Message::Delete)),
                            )
                        } else {
                            None
                        }
                    }))
                    .push(
                        widget::button::destructive(fl!("reset-to-defaults"))
                            .trailing_icon(icon_handle!("edit-undo-symbolic", 16))
                            .spacing(spacing.space_xs)
                            .on_press(Message::ToggleDialogPage(DialogPage::ResetPage(
                                Page::Layouts,
                            ))),
                    )
                    .spacing(spacing.space_xxs)
                    .apply(widget::container)
                    .class(cosmic::style::Container::Card)
                    .padding(spacing.space_xxs)
                    .into(),
            ),
            Page::Snapshots => Some(
                widget::row(vec![])
                    .push(widget::space::horizontal())
                    .push(
                        widget::button::standard(fl!("create-snapshot"))
                            .trailing_icon(icon_handle!("list-add-symbolic", 16))
                            .spacing(spacing.space_xs)
                            .on_press(Message::ToggleDialogPage(DialogPage::CreateSnapshot(
                                String::new(),
                            ))),
                    )
                    .spacing(spacing.space_xxs)
                    .apply(widget::container)
                    .class(cosmic::style::Container::Card)
                    .padding(spacing.space_xxs)
                    .into(),
            ),
            Page::Dock | Page::Panel | Page::Shortcuts => {
                let page = app.cosmic.nav_model.active_data::<Page>().copied()?;
                Some(
                    widget::row(vec![])
                        .push(widget::space::horizontal())
                        .push(
                            widget::button::destructive(fl!("reset-to-defaults"))
                                .trailing_icon(icon_handle!("edit-undo-symbolic", 16))
                                .spacing(spacing.space_xs)
                                .on_press(Message::ToggleDialogPage(DialogPage::ResetPage(page))),
                        )
                        .spacing(spacing.space_xxs)
                        .apply(widget::container)
                        .class(cosmic::style::Container::Card)
                        .padding(spacing.space_xxs)
                        .into(),
                )
            }
        }
    }
}
