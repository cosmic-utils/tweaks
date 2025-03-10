use crate::{
    app::style::{destructive_button, link_button, standard_button},
    fl,
};
use cosmic::{
    iced::{Alignment, Length},
    widget::{self, tooltip},
    Apply, Element,
};

use crate::core::icons;

use super::config::ColorScheme;

pub fn installed<'a>(
    color_scheme: &ColorScheme,
    selected: &ColorScheme,
) -> Element<'a, super::Message> {
    let theme = color_scheme.theme.clone().build();
    let spacing = cosmic::theme::active().cosmic().spacing;
    let color_scheme_name = color_scheme.name.clone();
    widget::button::custom(
        widget::column::with_children(vec![
            widget::row::with_children(vec![
                widget::horizontal_space().into(),
                widget::text(color_scheme_name).into(),
                widget::horizontal_space().into(),
            ])
            .padding(spacing.space_xxs)
            .into(),
            widget::row::with_children(vec![
                widget::container(widget::text(fl!("navigation")))
                    .padding(spacing.space_xxs)
                    .width(90.0)
                    .height(Length::Fill)
                    .class(crate::app::style::card(theme.clone()))
                    .into(),
                widget::horizontal_space().into(),
                widget::tooltip::tooltip(
                    icons::get_handle("symbolic-link-symbolic", 14)
                        .apply(widget::button::icon)
                        .class(link_button(theme.clone()))
                        .padding(spacing.space_xxs)
                        .on_press(super::Message::OpenContainingFolder(color_scheme.clone())),
                    widget::text(fl!("open-containing-folder")),
                    tooltip::Position::Bottom,
                )
                .into(),
                widget::tooltip::tooltip(
                    icons::get_handle("user-trash-symbolic", 14)
                        .apply(widget::button::icon)
                        .class(destructive_button(theme.clone()))
                        .padding(spacing.space_xxs)
                        .on_press(super::Message::DeleteColorScheme(color_scheme.clone())),
                    widget::text(fl!("delete-color-scheme")),
                    tooltip::Position::Bottom,
                )
                .into(),
            ])
            .align_y(Alignment::End)
            .spacing(spacing.space_xxs)
            .padding([0, spacing.space_xxs, spacing.space_xxs, spacing.space_xxs])
            .into(),
        ])
        .width(Length::Fixed(200.0))
        .height(Length::Fixed(160.0))
        .apply(widget::container)
        .class(crate::app::style::background(&theme)),
    )
    .selected(selected.name == color_scheme.name)
    .class(cosmic::style::Button::Image)
    .on_press(super::Message::SetColorScheme(color_scheme.clone()))
    .into()
}

pub fn available<'a>(color_scheme: &ColorScheme) -> Element<'a, super::Message> {
    let theme = color_scheme.theme.clone().build();
    let spacing = cosmic::theme::active().cosmic().spacing;
    let color_scheme_name = color_scheme.name.clone();
    let mut theme_caption = vec![widget::text(color_scheme_name).into()];

    if let Some(author) = &color_scheme.author {
        theme_caption.push(widget::text::caption(author.clone()).into());
    }

    widget::column::with_children(vec![
        widget::column::with_children(theme_caption)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .padding([spacing.space_xxs, spacing.space_none])
            .into(),
        widget::row::with_children(vec![
            widget::container(widget::text(fl!("navigation")))
                .padding(spacing.space_xxs)
                .width(90.0)
                .height(Length::Fill)
                .class(crate::app::style::card(theme.clone()))
                .into(),
            widget::horizontal_space().into(),
            widget::tooltip::tooltip(
                icons::get_handle("symbolic-link-symbolic", 14)
                    .apply(widget::button::icon)
                    .class(link_button(theme.clone()))
                    .padding(spacing.space_xxs)
                    .on_press(super::Message::OpenLink(color_scheme.link.clone())),
                widget::text(fl!("open-link")),
                cosmic::widget::tooltip::Position::Bottom,
            )
            .into(),
            widget::tooltip::tooltip(
                icons::get_handle("folder-download-symbolic", 14)
                    .apply(widget::button::icon)
                    .class(standard_button(theme.clone()))
                    .padding(spacing.space_xxs)
                    .on_press(super::Message::InstallColorScheme(color_scheme.clone())),
                widget::text(fl!("install-color-scheme")),
                cosmic::widget::tooltip::Position::Bottom,
            )
            .into(),
        ])
        .align_y(Alignment::End)
        .spacing(spacing.space_xxs)
        .padding([0, spacing.space_xxs, spacing.space_xxs, spacing.space_xxs])
        .into(),
    ])
    .width(240.0)
    .height(160.0)
    .apply(widget::container)
    .class(crate::app::style::background(&theme))
    .into()
}
