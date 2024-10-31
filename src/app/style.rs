use cosmic::{
    cosmic_theme::Theme,
    iced::{Background, Border, Color},
    iced_core::Shadow,
    widget::{self, button, container},
};

pub fn background<'a>(theme: Theme) -> cosmic::theme::Container<'a> {
    let corner_radii = cosmic::theme::active().cosmic().corner_radii;
    cosmic::theme::Container::custom(move |_| container::Style {
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

pub fn card<'a>(theme: Theme) -> cosmic::theme::Container<'a> {
    let theme = theme.clone();
    let corner_radii = cosmic::theme::active().cosmic().corner_radii;

    cosmic::theme::Container::custom(move |_| container::Style {
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

pub fn panel_style(theme: &cosmic::Theme) -> widget::container::Style {
    let theme = theme.cosmic();
    cosmic::widget::container::Style {
        icon_color: Some(Color::from(theme.background.on)),
        text_color: Some(Color::from(theme.background.on)),
        background: Some(Background::Color(theme.background.base.into())),
        border: Border {
            radius: theme.corner_radii.radius_0.into(),
            ..Default::default()
        },
        shadow: Shadow::default(),
    }
}

#[allow(dead_code)]
pub fn standard_button(theme: Theme) -> cosmic::theme::Button {
    let active_theme = theme.clone();
    let disabled_theme = theme.clone();
    let hovered_theme = theme.clone();
    let pressed_theme = theme.clone();
    let _corner_radii = cosmic::theme::active().cosmic().corner_radii;

    cosmic::theme::Button::Custom {
        active: Box::new(move |_active, _cosmic| button::Style {
            background: Some(cosmic::iced_core::Background::Color(
                active_theme.on_accent_color().into(),
            )),
            ..Default::default()
        }),
        disabled: Box::new(move |_cosmic| button::Style {
            background: Some(cosmic::iced_core::Background::Color(
                disabled_theme.on_accent_color().into(),
            )),
            ..Default::default()
        }),
        hovered: Box::new(move |_hovered, _cosmic| button::Style {
            background: Some(cosmic::iced_core::Background::Color(
                hovered_theme.on_accent_color().into(),
            )),
            ..Default::default()
        }),
        pressed: Box::new(move |_pressed, _cosmic| button::Style {
            background: Some(cosmic::iced_core::Background::Color(
                pressed_theme.on_accent_color().into(),
            )),
            ..Default::default()
        }),
    }
}
