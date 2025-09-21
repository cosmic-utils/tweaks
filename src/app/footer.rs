use cosmic::{
    Apply, Element,
    widget::{self},
};

use crate::app::{App, dialog::DialogPage};
use crate::app::{message::Message, pages::layouts::dialog::CreateLayoutDialog};
use crate::app::{page::Page, pages};

use super::Cosmic;
use crate::app::core::icons;
use crate::app::pages::{
    color_schemes::{self, Status, Tab},
    layouts::preview::LayoutPreview,
};
use crate::fl;

impl Cosmic {
    pub fn footer<'a>(app: &'a App) -> Option<Element<'a, Message>> {
        let spacing = cosmic::theme::spacing();

        match app.cosmic.nav_model.active_data::<Page>()? {
            Page::ColorSchemes => match app.color_schemes.model.active_data::<Tab>()? {
                Tab::Installed => Some(
                    widget::row()
                        .push(widget::horizontal_space())
                        .push(
                            widget::button::standard(fl!("save-current-color-scheme"))
                                .trailing_icon(icons::get_handle("arrow-into-box-symbolic", 16))
                                .spacing(spacing.space_xs)
                                .on_press(Message::ColorSchemes(Box::new(
                                    color_schemes::Message::SaveCurrentColorScheme(None),
                                ))),
                        )
                        .push(
                            widget::button::standard(fl!("import-color-scheme"))
                                .trailing_icon(icons::get_handle("document-save-symbolic", 16))
                                .spacing(spacing.space_xs)
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
                Tab::Available => Some(
                    widget::row()
                        .push(widget::horizontal_space())
                        .push(match app.color_schemes.status {
                            Status::Idle => widget::button::standard(fl!("show-more"))
                                .leading_icon(crate::app::core::icons::get_handle(
                                    "content-loading-symbolic",
                                    16,
                                ))
                                .on_press(Message::ColorSchemes(Box::new(
                                    color_schemes::Message::FetchAvailableColorSchemes(
                                        color_schemes::ColorSchemeProvider::CosmicThemes,
                                        app.color_schemes.limit,
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
            },
            Page::Layouts => Some(
                widget::row()
                    .push(widget::horizontal_space())
                    .push(
                        widget::button::standard(fl!("save-current-layout"))
                            .trailing_icon(icons::get_handle("arrow-into-box-symbolic", 16))
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
                            .trailing_icon(icons::get_handle("checkmark-symbolic", 16))
                            .spacing(spacing.space_xs)
                            .on_press(Message::Layouts(pages::layouts::Message::Apply))
                    }))
                    .push_maybe(app.layouts.selected_layout.as_ref().and_then(|selected| {
                        if selected.custom {
                            Some(
                                widget::button::standard(fl!("delete-layout"))
                                    .trailing_icon(icons::get_handle("recycling-bin-symbolic", 16))
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
                            .trailing_icon(icons::get_handle("list-add-symbolic", 16))
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
