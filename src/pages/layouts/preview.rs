use cosmic::{
    iced::{
        alignment::{Horizontal, Vertical},
        Length,
    },
    widget::{self, horizontal_space, vertical_space},
    Apply, Element,
};

use super::Message;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutPreview {
    panel: Option<PanelProperties>,
    dock: Option<PanelProperties>,
    dock_icons: u8,
    show_window: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PanelProperties {
    pub position: Position,
    pub extend: bool,
    pub size: f32,
}

impl PanelProperties {
    pub fn new(position: Position, extend: bool, size: f32) -> Self {
        Self {
            position,
            extend,
            size,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(unused)]
pub enum Position {
    Top,
    Bottom,
    Left,
    Right,
}

impl LayoutPreview {
    pub fn new(
        panel: Option<PanelProperties>,
        dock: Option<PanelProperties>,
        dock_icons: u8,
        show_window: bool,
    ) -> Self {
        Self {
            panel,
            dock,
            dock_icons,
            show_window,
        }
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        let column = widget::column().width(188).height(98);
        let row = widget::row().width(188).height(98);
        let spacing = cosmic::theme::spacing();

        let panel = widget::container(widget::text(""));

        let content: Element<_> = match (self.panel, self.dock) {
            (None, None) => column.into(),
            (None, Some(dock_props)) => {
                let extend_dock = if dock_props.extend {
                    Length::Fill
                } else {
                    Length::Shrink
                };

                let icons = (0..self.dock_icons)
                    .map(|_| square(dock_props.size - 5.0))
                    .collect();

                let icons: Element<_> =
                    if matches!(dock_props.position, Position::Top | Position::Bottom) {
                        widget::row::with_children(icons)
                            .spacing(spacing.space_xxs)
                            .align_y(Vertical::Center)
                            .into()
                    } else {
                        widget::column::with_children(icons)
                            .spacing(spacing.space_xxs)
                            .align_x(Horizontal::Center)
                            .into()
                    };

                let dock = widget::container(icons)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .padding(5)
                    .class(if dock_props.extend {
                        cosmic::style::Container::custom(crate::core::style::panel_style)
                    } else {
                        cosmic::style::Container::Background
                    });

                match dock_props.position {
                    Position::Top => column
                        .push(dock.width(extend_dock))
                        .align_x(Horizontal::Center)
                        .into(),
                    Position::Bottom => column
                        .push(vertical_space())
                        .push(dock.width(extend_dock))
                        .align_x(Horizontal::Center)
                        .into(),
                    Position::Left => row.push(dock).align_y(Vertical::Center).into(),
                    Position::Right => row
                        .push(horizontal_space())
                        .push(dock)
                        .align_y(Vertical::Center)
                        .into(),
                }
            }
            (Some(panel_props), None) => {
                let panel = panel.class(if panel_props.extend {
                    cosmic::style::Container::custom(crate::core::style::panel_style)
                } else {
                    cosmic::style::Container::Background
                });
                let extend_panel = if panel_props.extend {
                    Length::Fill
                } else {
                    Length::Shrink
                };
                match panel_props.position {
                    Position::Top => column
                        .push(panel.width(extend_panel).height(panel_props.size))
                        .align_x(Horizontal::Center)
                        .into(),
                    Position::Bottom => column
                        .push(vertical_space())
                        .push(panel.width(extend_panel).height(panel_props.size))
                        .align_x(Horizontal::Center)
                        .into(),
                    Position::Left => row
                        .push(panel.width(panel_props.size).height(extend_panel))
                        .align_y(Vertical::Center)
                        .into(),
                    Position::Right => row
                        .push(horizontal_space())
                        .push(panel.width(panel_props.size).height(extend_panel))
                        .align_y(Vertical::Center)
                        .into(),
                }
            }
            (Some(panel_props), Some(dock_props)) => {
                let panel = panel.class(if panel_props.extend {
                    cosmic::style::Container::custom(crate::core::style::panel_style)
                } else {
                    cosmic::style::Container::Background
                });
                let extend_panel = if panel_props.extend {
                    Length::Fill
                } else {
                    Length::Shrink
                };
                let extend_dock = if dock_props.extend {
                    Length::Fill
                } else {
                    Length::Shrink
                };

                let icons = (0..self.dock_icons)
                    .map(|_| square(dock_props.size - 5.0))
                    .collect();

                let icons: Element<_> =
                    if matches!(dock_props.position, Position::Top | Position::Bottom) {
                        widget::row::with_children(icons)
                            .spacing(spacing.space_xxs)
                            .align_y(Vertical::Center)
                            .into()
                    } else {
                        widget::column::with_children(icons)
                            .spacing(spacing.space_xxs)
                            .align_x(Horizontal::Center)
                            .into()
                    };

                let dock = widget::container(icons)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .padding(5)
                    .class(if dock_props.extend {
                        cosmic::style::Container::custom(crate::core::style::panel_style)
                    } else {
                        cosmic::style::Container::Background
                    });

                match (panel_props.position, dock_props.position) {
                    (Position::Top, Position::Top) => column
                        .push(panel.width(extend_panel).height(panel_props.size))
                        .push(dock.width(extend_dock))
                        .align_x(Horizontal::Center)
                        .into(),
                    (Position::Top, Position::Bottom) => column
                        .push(panel.width(extend_panel).height(panel_props.size))
                        .push(vertical_space())
                        .push(dock.width(extend_dock))
                        .align_x(Horizontal::Center)
                        .into(),
                    (Position::Top, Position::Left) => column
                        .push(panel.width(extend_panel).height(panel_props.size))
                        .push(
                            widget::row()
                                .push(dock.height(extend_dock))
                                .width(Length::Fill),
                        )
                        .align_x(Horizontal::Center)
                        .into(),
                    (Position::Top, Position::Right) => column
                        .push(panel.width(extend_panel).height(panel_props.size))
                        .push(
                            widget::row()
                                .push(horizontal_space())
                                .push(dock.height(extend_dock))
                                .width(Length::Fill),
                        )
                        .align_x(Horizontal::Center)
                        .into(),
                    (Position::Bottom, Position::Top) => column
                        .push(dock.width(extend_dock))
                        .push(vertical_space())
                        .push(panel.width(extend_panel).height(panel_props.size))
                        .align_x(Horizontal::Center)
                        .into(),
                    (Position::Bottom, Position::Bottom) => column
                        .push(vertical_space())
                        .push(panel.width(extend_panel).height(panel_props.size))
                        .push(dock.width(extend_dock))
                        .align_x(Horizontal::Center)
                        .into(),
                    (Position::Bottom, Position::Left) => column
                        .push(
                            widget::row()
                                .push(dock.height(extend_dock))
                                .width(Length::Fill),
                        )
                        .push(panel.width(extend_panel).height(panel_props.size))
                        .align_x(Horizontal::Center)
                        .into(),
                    (Position::Bottom, Position::Right) => column
                        .push(
                            widget::row()
                                .push(horizontal_space())
                                .push(dock.height(extend_dock)),
                        )
                        .push(panel.width(extend_panel).height(panel_props.size))
                        .align_x(Horizontal::Center)
                        .into(),
                    (Position::Left, Position::Top) => row
                        .push(panel.width(panel_props.size).height(extend_panel))
                        .push(
                            widget::column()
                                .push(dock.width(extend_dock))
                                .align_x(Horizontal::Center),
                        )
                        .align_y(Vertical::Center)
                        .into(),
                    (Position::Left, Position::Bottom) => row
                        .push(panel.width(panel_props.size).height(extend_panel))
                        .push(
                            widget::column()
                                .push(horizontal_space())
                                .push(dock.width(extend_dock))
                                .align_x(Horizontal::Center),
                        )
                        .align_y(Vertical::Center)
                        .into(),
                    (Position::Left, Position::Left) => row
                        .push(panel.width(panel_props.size).height(extend_panel))
                        .push(dock.height(extend_dock))
                        .align_y(Vertical::Center)
                        .into(),
                    (Position::Left, Position::Right) => row
                        .push(panel.width(panel_props.size).height(extend_panel))
                        .push(horizontal_space())
                        .push(dock.height(extend_dock))
                        .align_y(Vertical::Center)
                        .into(),
                    (Position::Right, Position::Top) => row
                        .push(
                            widget::column()
                                .push(dock.width(extend_dock))
                                .align_x(Horizontal::Center),
                        )
                        .push(horizontal_space())
                        .push(panel.width(panel_props.size).height(extend_panel))
                        .align_y(Vertical::Center)
                        .into(),
                    (Position::Right, Position::Bottom) => row
                        .push(
                            widget::column()
                                .push(vertical_space())
                                .push(dock.width(extend_dock))
                                .align_x(Horizontal::Center),
                        )
                        .push(horizontal_space())
                        .push(panel.width(panel_props.size).height(extend_panel))
                        .align_y(Vertical::Center)
                        .into(),
                    (Position::Right, Position::Left) => row
                        .push(dock.height(extend_dock))
                        .push(horizontal_space())
                        .push(panel.width(panel_props.size).height(extend_panel))
                        .align_y(Vertical::Center)
                        .into(),
                    (Position::Right, Position::Right) => row
                        .push(horizontal_space())
                        .push(dock.height(extend_dock))
                        .push(panel.width(panel_props.size).height(extend_panel))
                        .align_y(Vertical::Center)
                        .into(),
                }
            }
        };

        content
            .apply(widget::container)
            .class(cosmic::style::Container::Secondary)
            .padding(spacing.space_xxxs)
            .into()
    }
}

pub fn square<'a>(size: f32) -> Element<'a, Message> {
    widget::container(widget::text(""))
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .class(cosmic::style::Container::Secondary)
        .into()
}
