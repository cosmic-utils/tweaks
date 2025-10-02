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
use crate::fl;

impl Cosmic {
    pub fn footer<'a>(app: &'a App) -> Option<Element<'a, Message>> {
        let spacing = cosmic::theme::spacing();

        match app.cosmic.nav_model.active_data::<Page>()? {
            Page::ColorSchemes => app
                .color_schemes
                .footer()
                .map(|elem| elem.map(|message| Message::ColorSchemes(Box::new(message)))),
            Page::Layouts => Some(
                widget::row()
                    .push(widget::horizontal_space())
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
                    .spacing(spacing.space_xxs)
                    .apply(widget::container)
                    .class(cosmic::style::Container::Card)
                    .padding(spacing.space_xxs)
                    .into(),
            ),
            Page::Snapshots => Some(
                widget::row()
                    .push(widget::horizontal_space())
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
            _ => None,
        }
    }
}
