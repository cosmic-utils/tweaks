use crate::fl;
use cosmic::{
    cosmic_theme::Theme,
    iced::{Alignment, Border, Color, Length},
    iced_core::Shadow,
    widget::{self, button, container, tooltip},
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
                widget::horizontal_space(Length::Fill).into(),
                widget::text(color_scheme_name).into(),
                widget::horizontal_space(Length::Fill).into(),
            ])
            .padding(spacing.space_xxs)
            .into(),
            widget::row::with_children(vec![
                widget::container(widget::text("Navigation"))
                    .padding(spacing.space_xxs)
                    .width(Length::Fixed(100.0))
                    .height(Length::Fill)
                    .style(card(&theme))
                    .into(),
                widget::horizontal_space(Length::Fill).into(),
                widget::tooltip::tooltip(
                    icons::get_handle("symbolic-link-symbolic", 14)
                        .apply(widget::button::icon)
                        .style(cosmic::theme::Button::Link)
                        .padding(spacing.space_xxs)
                        .on_press(super::Message::OpenContainingFolder(color_scheme.clone())),
                    fl!("open-containing-folder"),
                    tooltip::Position::Bottom,
                )
                .into(),
                widget::tooltip::tooltip(
                    icons::get_handle("user-trash-symbolic", 14)
                        .apply(widget::button::icon)
                        .style(cosmic::theme::Button::Destructive)
                        .padding(spacing.space_xxs)
                        .on_press(super::Message::DeleteColorScheme(color_scheme.clone())),
                    fl!("delete-color-scheme"),
                    tooltip::Position::Bottom,
                )
                .into(),
            ])
            .align_items(Alignment::End)
            .spacing(spacing.space_xxs)
            .padding([0, spacing.space_xxs, spacing.space_xxs, spacing.space_xxs])
            .into(),
        ])
        .width(Length::Fixed(200.0))
        .height(Length::Fixed(160.0))
        .apply(widget::container)
        .style(background(&theme)),
    )
    .selected(selected.name == color_scheme.name)
    .style(cosmic::theme::Button::Image)
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

    widget::button::custom(
        widget::column::with_children(vec![
            widget::column::with_children(theme_caption)
                .width(Length::Fill)
                .align_items(Alignment::Center)
                .padding([spacing.space_xxs, spacing.space_none])
                .into(),
            widget::row::with_children(vec![
                widget::container(widget::text("Navigation"))
                    .padding(spacing.space_xxs)
                    .width(Length::Fixed(100.0))
                    .height(Length::Fill)
                    .style(card(&theme))
                    .into(),
                widget::horizontal_space(Length::Fill).into(),
                widget::tooltip::tooltip(
                    icons::get_handle("symbolic-link-symbolic", 14)
                        .apply(widget::button::icon)
                        .style(cosmic::theme::Button::Link)
                        .padding(spacing.space_xxs)
                        .on_press(crate::app::Message::ColorSchemes(Box::new(
                            super::Message::OpenLink(color_scheme.link.clone()),
                        ))),
                    fl!("open-link"),
                    cosmic::widget::tooltip::Position::Bottom,
                )
                .into(),
                widget::tooltip::tooltip(
                    icons::get_handle("folder-download-symbolic", 14)
                        .apply(widget::button::icon)
                        .style(cosmic::theme::Button::Suggested)
                        .padding(spacing.space_xxs)
                        .on_press(crate::app::Message::ColorSchemes(Box::new(
                            super::Message::InstallColorScheme(color_scheme.clone()),
                        ))),
                    fl!("install-color-scheme"),
                    cosmic::widget::tooltip::Position::Bottom,
                )
                .into(),
            ])
            .align_items(Alignment::End)
            .spacing(spacing.space_xxs)
            .padding([0, spacing.space_xxs, spacing.space_xxs, spacing.space_xxs])
            .into(),
        ])
        .height(Length::Fixed(160.0))
        .apply(widget::container)
        .style(background(&theme)),
    )
    .style(cosmic::theme::Button::Image)
    .on_press(crate::app::Message::ColorSchemes(Box::new(
        super::Message::SetColorScheme(color_scheme.clone()),
    )))
    .into()
}

pub fn background(theme: &Theme) -> cosmic::theme::Container {
    let theme = theme.clone();
    let corner_radii = cosmic::theme::active().cosmic().corner_radii;
    cosmic::theme::Container::custom(move |_| container::Appearance {
        icon_color: Some(Color::from(theme.background.on)),
        text_color: Some(Color::from(theme.background.on)),
        background: Some(cosmic::iced::Background::Color(
            theme.background.base.into(),
        )),
        border: Border {
            radius: corner_radii.radius_xs.into(),
            ..Default::default()
        },
        shadow: Shadow::default(),
    })
}

pub fn card(theme: &Theme) -> cosmic::theme::Container {
    let theme = theme.clone();
    let corner_radii = cosmic::theme::active().cosmic().corner_radii;

    cosmic::theme::Container::custom(move |_| container::Appearance {
        icon_color: Some(Color::from(theme.primary.component.on)),
        text_color: Some(Color::from(theme.primary.component.on)),
        background: Some(cosmic::iced::Background::Color(
            theme.primary.component.base.into(),
        )),
        border: Border {
            radius: corner_radii.radius_s.into(),
            ..Default::default()
        },
        shadow: Shadow::default(),
    })
}

#[allow(dead_code)]
pub fn standard_button(theme: &Theme) -> cosmic::theme::Button {
    let active_theme = theme.clone();
    let disabled_theme = theme.clone();
    let hovered_theme = theme.clone();
    let pressed_theme = theme.clone();
    let _corner_radii = cosmic::theme::active().cosmic().corner_radii;

    cosmic::theme::Button::Custom {
        active: Box::new(move |_active, _cosmic| button::Appearance {
            background: Some(cosmic::iced_core::Background::Color(
                active_theme.on_accent_color().into(),
            )),
            ..Default::default()
        }),
        disabled: Box::new(move |_cosmic| button::Appearance {
            background: Some(cosmic::iced_core::Background::Color(
                disabled_theme.on_accent_color().into(),
            )),
            ..Default::default()
        }),
        hovered: Box::new(move |_hovered, _cosmic| button::Appearance {
            background: Some(cosmic::iced_core::Background::Color(
                hovered_theme.on_accent_color().into(),
            )),
            ..Default::default()
        }),
        pressed: Box::new(move |_pressed, _cosmic| button::Appearance {
            background: Some(cosmic::iced_core::Background::Color(
                pressed_theme.on_accent_color().into(),
            )),
            ..Default::default()
        }),
    }
}
