use super::config::ColorScheme;
use crate::app::core::{
    icons,
    style::{destructive_button, link_button, standard_button},
};
use crate::fl;
use cosmic::{
    Apply, Element,
    iced::{Alignment, Length},
    widget::{self, tooltip},
};

pub fn installed<'a>(
    color_scheme: &ColorScheme,
    selected: &ColorScheme,
    spacing: &cosmic::cosmic_theme::Spacing,
    item_width: usize,
) -> Element<'a, super::Message> {
    let theme = color_scheme.theme.clone().build();
    let color_scheme_name = color_scheme.name.clone();
    widget::column()
        .push(
            widget::row()
                .push(widget::horizontal_space())
                .push(widget::text(color_scheme_name))
                .push(widget::horizontal_space())
                .padding(spacing.space_xxs),
        )
        .push(
            widget::row()
                .push(
                    widget::container(widget::text(fl!("navigation")))
                        .padding(spacing.space_xxs)
                        .width(90.0)
                        .height(Length::Fill)
                        .class(crate::app::core::style::card(theme.clone())),
                )
                .push(widget::horizontal_space())
                .push(widget::tooltip::tooltip(
                    icons::get_handle("selection-mode-symbolic", 14)
                        .apply(widget::button::icon)
                        .class(link_button(theme.clone()))
                        .padding(spacing.space_xxs)
                        .selected(color_scheme.name == selected.name)
                        .class(if selected.name == color_scheme.name {
                            cosmic::style::Button::Standard
                        } else {
                            cosmic::style::Button::Icon
                        })
                        .on_press(super::Message::SetColorScheme(color_scheme.clone())),
                    widget::text(fl!("set-color-scheme")),
                    tooltip::Position::Bottom,
                ))
                .push(widget::tooltip::tooltip(
                    icons::get_handle("symbolic-link-symbolic", 14)
                        .apply(widget::button::icon)
                        .class(link_button(theme.clone()))
                        .padding(spacing.space_xxs)
                        .on_press(super::Message::OpenContainingFolder(color_scheme.clone())),
                    widget::text(fl!("open-containing-folder")),
                    tooltip::Position::Bottom,
                ))
                .push(widget::tooltip::tooltip(
                    icons::get_handle("user-trash-symbolic", 14)
                        .apply(widget::button::icon)
                        .class(destructive_button(theme.clone()))
                        .padding(spacing.space_xxs)
                        .on_press(super::Message::DeleteColorScheme(color_scheme.clone())),
                    widget::text(fl!("delete-color-scheme")),
                    tooltip::Position::Bottom,
                ))
                .align_y(Alignment::End)
                .spacing(spacing.space_xxs)
                .padding([0, spacing.space_xxs, spacing.space_xxs, spacing.space_xxs]),
        )
        .width(item_width as f32)
        .height(160.)
        .apply(widget::container)
        .class(crate::app::core::style::background(&theme))
        .into()
}

pub fn available<'a>(
    color_scheme: &'a ColorScheme,
    spacing: &cosmic::cosmic_theme::Spacing,
    item_width: usize,
) -> Element<'a, super::Message> {
    let theme = color_scheme.theme.clone().build();
    widget::column()
        .push(
            widget::column()
                .push(widget::text(&color_scheme.name))
                .push_maybe(
                    color_scheme
                        .author
                        .as_ref()
                        .map(|author| widget::text::caption(author.clone())),
                )
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .padding([spacing.space_xxs, spacing.space_none]),
        )
        .push(
            widget::row()
                .push(
                    widget::container(widget::text(fl!("navigation")))
                        .padding(spacing.space_xxs)
                        .width(90.0)
                        .height(Length::Fill)
                        .class(crate::app::core::style::card(theme.clone())),
                )
                .push(widget::horizontal_space())
                .push(widget::tooltip::tooltip(
                    icons::get_handle("symbolic-link-symbolic", 14)
                        .apply(widget::button::icon)
                        .class(link_button(theme.clone()))
                        .padding(spacing.space_xxs)
                        .on_press(super::Message::OpenLink(color_scheme.link.clone())),
                    widget::text(fl!("open-link")),
                    cosmic::widget::tooltip::Position::Bottom,
                ))
                .push(widget::tooltip::tooltip(
                    icons::get_handle("folder-download-symbolic", 14)
                        .apply(widget::button::icon)
                        .class(standard_button(theme.clone()))
                        .padding(spacing.space_xxs)
                        .on_press(super::Message::InstallColorScheme(color_scheme.clone())),
                    widget::text(fl!("install-color-scheme")),
                    cosmic::widget::tooltip::Position::Bottom,
                ))
                .align_y(Alignment::End)
                .spacing(spacing.space_xxs)
                .padding([0, spacing.space_xxs, spacing.space_xxs, spacing.space_xxs]),
        )
        .width(item_width as f32)
        .height(160.)
        .apply(widget::container)
        .class(crate::app::core::style::background(&theme))
        .into()
}
