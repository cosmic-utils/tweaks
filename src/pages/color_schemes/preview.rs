use crate::fl;
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
                widget::container(widget::text("Navigation"))
                    .padding(spacing.space_xxs)
                    .width(Length::Fixed(100.0))
                    .height(Length::Fill)
                    .class(crate::app::style::card(theme.clone()))
                    .into(),
                widget::horizontal_space().into(),
                widget::tooltip::tooltip(
                    icons::get_handle("symbolic-link-symbolic", 14)
                        .apply(widget::button::icon)
                        .class(cosmic::style::Button::Link)
                        .padding(spacing.space_xxs)
                        .on_press(super::Message::OpenContainingFolder(color_scheme.clone())),
                    widget::text(fl!("open-containing-folder")),
                    tooltip::Position::Bottom,
                )
                .into(),
                widget::tooltip::tooltip(
                    icons::get_handle("user-trash-symbolic", 14)
                        .apply(widget::button::icon)
                        .class(cosmic::style::Button::Destructive)
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
        .class(crate::app::style::background(theme.clone())),
    )
    .selected(selected.name == color_scheme.name)
    .class(cosmic::style::Button::Image)
    .on_press(super::Message::SetColorScheme(color_scheme.clone()))
    .into()
}

pub fn available<'a>(color_scheme: &ColorScheme) -> Element<'a, crate::app::Message> {
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
            widget::container(widget::text("Navigation"))
                .padding(spacing.space_xxs)
                .width(Length::Fill)
                .height(Length::Fill)
                .class(crate::app::style::card(theme.clone()))
                .into(),
            widget::horizontal_space().into(),
            widget::tooltip::tooltip(
                icons::get_handle("symbolic-link-symbolic", 14)
                    .apply(widget::button::icon)
                    .class(cosmic::style::Button::Link)
                    .padding(spacing.space_xxs)
                    .on_press(crate::app::Message::ColorSchemes(Box::new(
                        super::Message::OpenLink(color_scheme.link.clone()),
                    ))),
                widget::text(fl!("open-link")),
                cosmic::widget::tooltip::Position::Bottom,
            )
            .into(),
            widget::tooltip::tooltip(
                icons::get_handle("folder-download-symbolic", 14)
                    .apply(widget::button::icon)
                    .class(cosmic::style::Button::Suggested)
                    .padding(spacing.space_xxs)
                    .on_press(crate::app::Message::ColorSchemes(Box::new(
                        super::Message::InstallColorScheme(color_scheme.clone()),
                    ))),
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
    .height(Length::Fixed(160.0))
    .apply(widget::container)
    .class(crate::app::style::background(theme.clone()))
    .into()
}
