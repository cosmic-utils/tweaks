use cosmic::{
    cosmic_theme::Theme,
    iced::{Border, Color, Length},
    iced_core::Shadow,
    widget,
    widget::{button, container},
    Apply, Element,
};

use super::config::ColorScheme;

pub fn view<'a>(color_scheme: &ColorScheme, selected: &ColorScheme) -> Element<'a, super::Message> {
    let theme = color_scheme.theme.clone().build();
    let spacing = cosmic::theme::active().cosmic().spacing;
    let color_scheme_name = color_scheme.name.clone();
    widget::button(
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
                widget::button(widget::text("Button"))
                    // .style(standard_button(&theme))
                    .into(),
            ])
            .padding([0, spacing.space_xxs, spacing.space_xxs, spacing.space_xxs])
            .into(),
        ])
        .width(Length::Fixed(200.0))
        .height(Length::Fixed(160.0))
        .apply(widget::container)
        .style(background(&theme)),
    )
    .selected(selected == color_scheme)
    .style(cosmic::theme::Button::Image)
    .on_press(super::Message::SetColorScheme(color_scheme.clone()))
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

pub fn standard_button(theme: &Theme) -> cosmic::theme::Button {
    let active_theme = theme.clone();
    let disabled_theme = theme.clone();
    let hovered_theme = theme.clone();
    let pressed_theme = theme.clone();
    let corner_radii = cosmic::theme::active().cosmic().corner_radii;

    cosmic::theme::Button::Custom {
        active: Box::new(move |active, cosmic| button::Appearance {
            background: Some(cosmic::iced_core::Background::Color(
                active_theme.on_accent_color().into(),
            )),
            ..Default::default()
        }),
        disabled: Box::new(move |cosmic| button::Appearance {
            background: Some(cosmic::iced_core::Background::Color(
                disabled_theme.on_accent_color().into(),
            )),
            ..Default::default()
        }),
        hovered: Box::new(move |hovered, cosmic| button::Appearance {
            background: Some(cosmic::iced_core::Background::Color(
                hovered_theme.on_accent_color().into(),
            )),
            ..Default::default()
        }),
        pressed: Box::new(move |pressed, cosmic| button::Appearance {
            background: Some(cosmic::iced_core::Background::Color(
                pressed_theme.on_accent_color().into(),
            )),
            ..Default::default()
        }),
    }
}
