use cosmic::{
    cosmic_theme::{Component, Theme},
    iced::{Background, Border, Color},
    iced_core::Shadow,
    theme::{Button, TRANSPARENT_COMPONENT},
    widget::{self, container},
};

pub fn background<'a>(theme: &Theme) -> cosmic::theme::Container<'a> {
    let theme = theme.clone();
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

pub fn standard_button(theme: Theme) -> Button {
    let theme_active = theme.clone();
    let theme_disabled = theme.clone();
    let theme_hovered = theme.clone();
    let theme_pressed = theme;

    Button::Custom {
        active: Box::new(move |_, _| {
            appearance(
                &theme_active,
                false,
                false,
                false,
                Button::Standard,
                |component| {
                    let text_color = Some(component.on.into());
                    (component.hover.into(), text_color, text_color)
                },
            )
        }),
        disabled: Box::new(move |_| {
            appearance(
                &theme_disabled,
                false,
                false,
                true,
                Button::Standard,
                |component| {
                    let mut background = Color::from(component.base);
                    background.a *= 0.5;
                    (
                        background,
                        Some(component.on_disabled.into()),
                        Some(component.on_disabled.into()),
                    )
                },
            )
        }),
        hovered: Box::new(move |_, _| {
            appearance(
                &theme_hovered,
                false,
                false,
                false,
                Button::Standard,
                |component| {
                    let text_color = Some(component.on.into());

                    (component.hover.into(), text_color, text_color)
                },
            )
        }),
        pressed: Box::new(move |_, _| {
            appearance(
                &theme_pressed,
                false,
                false,
                false,
                Button::Standard,
                |component| {
                    let text_color = Some(component.on.into());

                    (component.pressed.into(), text_color, text_color)
                },
            )
        }),
    }
}

pub fn destructive_button(theme: Theme) -> Button {
    let theme_active = theme.clone();
    let theme_disabled = theme.clone();
    let theme_hovered = theme.clone();
    let theme_pressed = theme;

    Button::Custom {
        active: Box::new(move |_, _| {
            appearance(
                &theme_active,
                false,
                false,
                false,
                Button::Destructive,
                |component| {
                    let text_color = Some(component.on.into());
                    (component.hover.into(), text_color, text_color)
                },
            )
        }),
        disabled: Box::new(move |_| {
            appearance(
                &theme_disabled,
                false,
                false,
                true,
                Button::Destructive,
                |component| {
                    let mut background = Color::from(component.base);
                    background.a *= 0.5;
                    (
                        background,
                        Some(component.on_disabled.into()),
                        Some(component.on_disabled.into()),
                    )
                },
            )
        }),
        hovered: Box::new(move |_, _| {
            appearance(
                &theme_hovered,
                false,
                false,
                false,
                Button::Destructive,
                |component| {
                    let text_color = Some(component.on.into());

                    (component.hover.into(), text_color, text_color)
                },
            )
        }),
        pressed: Box::new(move |_, _| {
            appearance(
                &theme_pressed,
                false,
                false,
                false,
                Button::Destructive,
                |component| {
                    let text_color = Some(component.on.into());

                    (component.pressed.into(), text_color, text_color)
                },
            )
        }),
    }
}

pub fn link_button(theme: Theme) -> Button {
    let theme_active = theme.clone();
    let theme_disabled = theme.clone();
    let theme_hovered = theme.clone();
    let theme_pressed = theme;

    Button::Custom {
        active: Box::new(move |_, _| {
            appearance(
                &theme_active,
                false,
                false,
                false,
                Button::Link,
                |component| {
                    let text_color = Some(component.on.into());
                    (component.hover.into(), text_color, text_color)
                },
            )
        }),
        disabled: Box::new(move |_| {
            appearance(
                &theme_disabled,
                false,
                false,
                true,
                Button::Link,
                |component| {
                    let mut background = Color::from(component.base);
                    background.a *= 0.5;
                    (
                        background,
                        Some(component.on_disabled.into()),
                        Some(component.on_disabled.into()),
                    )
                },
            )
        }),
        hovered: Box::new(move |_, _| {
            appearance(
                &theme_hovered,
                false,
                false,
                false,
                Button::Link,
                |component| {
                    let text_color = Some(component.on.into());

                    (component.hover.into(), text_color, text_color)
                },
            )
        }),
        pressed: Box::new(move |_, _| {
            appearance(
                &theme_pressed,
                false,
                false,
                false,
                Button::Link,
                |component| {
                    let text_color = Some(component.on.into());

                    (component.pressed.into(), text_color, text_color)
                },
            )
        }),
    }
}

pub fn appearance(
    theme: &Theme,
    focused: bool,
    selected: bool,
    disabled: bool,
    style: Button,
    color: impl Fn(&Component) -> (Color, Option<Color>, Option<Color>),
) -> widget::button::Style {
    let cosmic = theme;
    let mut corner_radii = &cosmic.corner_radii.radius_xl;
    let mut appearance = widget::button::Style::new();

    match style {
        Button::Standard
        | Button::Text
        | Button::Suggested
        | Button::Destructive
        | Button::Transparent => {
            let style_component = match style {
                Button::Standard => &cosmic.button,
                Button::Text => &cosmic.text_button,
                Button::Suggested => &cosmic.accent_button,
                Button::Destructive => &cosmic.destructive_button,
                Button::Transparent => &TRANSPARENT_COMPONENT,
                _ => return appearance,
            };

            let (background, text, icon) = color(style_component);
            appearance.background = Some(Background::Color(background));
            if !matches!(style, Button::Standard) {
                appearance.text_color = text;
                appearance.icon_color = icon;
            }
        }

        Button::Icon | Button::IconVertical | Button::HeaderBar | Button::NavToggle => {
            if matches!(style, Button::IconVertical) {
                corner_radii = &cosmic.corner_radii.radius_m;
                if selected {
                    appearance.overlay = Some(Background::Color(Color::from(
                        cosmic.icon_button.selected_state_color(),
                    )));
                }
            }
            if matches!(style, Button::NavToggle) {
                corner_radii = &cosmic.corner_radii.radius_s;
            }

            let (background, text, icon) = color(&cosmic.icon_button);
            appearance.background = Some(Background::Color(background));
            appearance.icon_color = if disabled { icon } else { None };
            appearance.text_color = if disabled { text } else { None };
        }

        Button::Image => {
            appearance.background = None;
            appearance.text_color = Some(cosmic.accent.base.into());
            appearance.icon_color = Some(cosmic.accent.base.into());

            corner_radii = &cosmic.corner_radii.radius_s;
            appearance.border_radius = (*corner_radii).into();

            if focused || selected {
                appearance.border_width = 2.0;
                appearance.border_color = cosmic.accent.base.into();
            }

            return appearance;
        }

        Button::Link => {
            appearance.background = None;
            appearance.icon_color = Some(cosmic.accent.base.into());
            appearance.text_color = Some(cosmic.accent.base.into());
            corner_radii = &cosmic.corner_radii.radius_0;
        }

        Button::Custom { .. } => (),
        Button::AppletMenu => {
            let (background, _, _) = color(&cosmic.text_button);
            appearance.background = Some(Background::Color(background));

            appearance.icon_color = Some(cosmic.background.on.into());
            appearance.text_color = Some(cosmic.background.on.into());
            corner_radii = &cosmic.corner_radii.radius_0;
        }
        Button::AppletIcon => {
            let (background, _, _) = color(&cosmic.text_button);
            appearance.background = Some(Background::Color(background));

            appearance.icon_color = Some(cosmic.background.on.into());
            appearance.text_color = Some(cosmic.background.on.into());
        }
        Button::MenuFolder => {
            let component = &cosmic.background.component;
            let (background, _, _) = color(component);
            appearance.background = Some(Background::Color(background));
            appearance.icon_color = Some(component.on.into());
            appearance.text_color = Some(component.on.into());
            corner_radii = &cosmic.corner_radii.radius_s;
        }
        Button::MenuItem => {
            let (background, text, icon) = color(&cosmic.background.component);
            appearance.background = Some(Background::Color(background));
            appearance.icon_color = icon;
            appearance.text_color = text;
            corner_radii = &cosmic.corner_radii.radius_s;
        }
        Button::MenuRoot => {
            appearance.background = None;
            appearance.icon_color = None;
            appearance.text_color = None;
        }

        Button::ListItem => {
            corner_radii = &[0.0; 4];
            let (background, text, icon) = color(&cosmic.background.component);

            if selected {
                appearance.background =
                    Some(Background::Color(cosmic.primary.component.hover.into()));
                appearance.icon_color = Some(cosmic.accent.base.into());
                appearance.text_color = Some(cosmic.accent.base.into());
            } else {
                appearance.background = Some(Background::Color(background));
                appearance.icon_color = icon;
                appearance.text_color = text;
            }
        }
    }

    appearance.border_radius = (*corner_radii).into();

    if focused {
        appearance.outline_width = 1.0;
        appearance.outline_color = cosmic.accent.base.into();
        appearance.border_width = 2.0;
        appearance.border_color = Color::TRANSPARENT;
    }

    appearance
}
