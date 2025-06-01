use cosmic::{
    widget::{self},
    Apply, Element,
};

use crate::app::message::Message;
use crate::app::page::Page;
use crate::app::{dialog::DialogPage, App};

use crate::{
    core::icons,
    fl,
    pages::{
        color_schemes::{self, Status, Tab},
        layouts::preview::LayoutPreview,
    },
};

use super::Cosmic;

impl Cosmic {
    pub fn footer(app: &App) -> Option<Element<Message>> {
        let spacing = cosmic::theme::spacing();

        match app.cosmic.nav_model.active_data::<Page>() {
            Some(Page::ColorSchemes) => match app.color_schemes.model.active_data::<Tab>() {
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
                        .push(match app.color_schemes.status {
                            Status::Idle => widget::button::standard(fl!("show-more"))
                                .leading_icon(crate::core::icons::get_handle(
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
                None => None,
            },
            Some(Page::Layouts) => Some(
                widget::row()
                    .push(widget::horizontal_space())
                    .push(
                        widget::button::standard(fl!("save-current-layout"))
                            .trailing_icon(icons::get_handle("arrow-into-box-symbolic", 16))
                            .on_press(Message::ToggleDialogPage(DialogPage::CreateLayout(
                                String::new(),
                                LayoutPreview::default(),
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
