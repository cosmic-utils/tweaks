use cosmic::{
    Apply, Element, Theme,
    iced::{
        Length,
        alignment::{Horizontal, Vertical},
    },
    widget::{self, horizontal_space, vertical_space},
};
use serde::{Deserialize, Serialize};

use crate::fl;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct LayoutPreview {
    pub panel: PanelProperties,
    pub dock: PanelProperties,
    pub dock_icons: u8,
    pub show_window: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PanelProperties {
    pub position: Position,
    pub extend: bool,
    pub hidden: bool,
    pub size: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Position {
    Top,
    Bottom,
    Left,
    Right,
}

impl LayoutPreview {
    pub fn view<'a, Message: Clone + 'a>(
        &self,
        spacing: &cosmic::cosmic_theme::Spacing,
        height: u16,
    ) -> Element<'a, Message> {
        let column = widget::column().height(height).width(Length::Fill);
        let row = widget::row().height(height);

        let panel = widget::container(match self.panel.position {
            Position::Top | Position::Bottom => Element::from(
                widget::row()
                    .push(Self::square(self.panel.size as f32 - 5.0))
                    .push_maybe(if self.panel.extend {
                        Some(horizontal_space())
                    } else {
                        None
                    })
                    .push(Self::square(self.panel.size as f32 - 5.0))
                    .spacing(spacing.space_xxs)
                    .align_y(Vertical::Center),
            ),
            Position::Left | Position::Right => widget::column()
                .push(Self::square(self.panel.size as f32 - 5.0))
                .push_maybe(if self.panel.extend {
                    Some(vertical_space())
                } else {
                    None
                })
                .push(Self::square(self.panel.size as f32 - 5.0))
                .spacing(spacing.space_xxs)
                .align_x(Horizontal::Center)
                .into(),
        })
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .padding(5);

        let content: Element<_> = match (self.panel.hidden, self.dock.hidden) {
            (true, true) => column.into(),
            (true, false) => {
                let extend_dock = if self.dock.extend {
                    Length::Fill
                } else {
                    Length::Shrink
                };

                let icons: Vec<Element<_>> = (0..self.dock_icons)
                    .map(|_| Self::square::<Message>(self.dock.size as f32 - 5.0).into())
                    .collect();

                let icons: Element<_> =
                    if matches!(self.dock.position, Position::Top | Position::Bottom) {
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
                    .class(if self.dock.extend {
                        cosmic::style::Container::custom(crate::app::core::style::panel_style)
                    } else {
                        cosmic::style::Container::Background
                    });

                match self.dock.position {
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
            (false, true) => {
                let panel = panel.class(if self.panel.extend {
                    cosmic::style::Container::custom(crate::app::core::style::panel_style)
                } else {
                    cosmic::style::Container::Background
                });
                let extend_panel = if self.panel.extend {
                    Length::Fill
                } else {
                    Length::Shrink
                };
                match self.panel.position {
                    Position::Top => column
                        .push(panel.width(extend_panel).height(self.panel.size as f32))
                        .align_x(Horizontal::Center)
                        .into(),
                    Position::Bottom => column
                        .push(vertical_space())
                        .push(panel.width(extend_panel).height(self.panel.size as f32))
                        .align_x(Horizontal::Center)
                        .into(),
                    Position::Left => row
                        .push(panel.width(self.panel.size as f32).height(extend_panel))
                        .push(horizontal_space())
                        .align_y(Vertical::Center)
                        .into(),
                    Position::Right => row
                        .push(horizontal_space())
                        .push(panel.width(self.panel.size as f32).height(extend_panel))
                        .align_y(Vertical::Center)
                        .into(),
                }
            }
            (false, false) => {
                let panel = panel.class(if self.panel.extend {
                    cosmic::style::Container::custom(crate::app::core::style::panel_style)
                } else {
                    cosmic::style::Container::Background
                });
                let extend_panel = if self.panel.extend {
                    Length::Fill
                } else {
                    Length::Shrink
                };
                let extend_dock = if self.dock.extend {
                    Length::Fill
                } else {
                    Length::Shrink
                };

                let icons: Vec<Element<_>> = (0..self.dock_icons)
                    .map(|_| Self::square::<Message>(self.dock.size as f32 - 5.0).into())
                    .collect();

                let icons: Element<_> =
                    if matches!(self.dock.position, Position::Top | Position::Bottom) {
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
                    .class(if self.dock.extend {
                        cosmic::style::Container::custom(crate::app::core::style::panel_style)
                    } else {
                        cosmic::style::Container::Background
                    });

                match (self.panel.position, self.dock.position) {
                    (Position::Top, Position::Top) => column
                        .push(panel.width(extend_panel).height(self.panel.size as f32))
                        .push(horizontal_space())
                        .push(dock.width(extend_dock))
                        .align_x(Horizontal::Center)
                        .into(),
                    (Position::Top, Position::Bottom) => column
                        .push(panel.width(extend_panel).height(self.panel.size as f32))
                        .push(vertical_space())
                        .push(horizontal_space())
                        .push(dock.width(extend_dock))
                        .align_x(Horizontal::Center)
                        .into(),
                    (Position::Top, Position::Left) => column
                        .push(panel.width(extend_panel).height(self.panel.size as f32))
                        .push(
                            widget::row()
                                .push(dock.height(extend_dock))
                                .width(Length::Fill),
                        )
                        .align_x(Horizontal::Center)
                        .into(),
                    (Position::Top, Position::Right) => column
                        .push(panel.width(extend_panel).height(self.panel.size as f32))
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
                        .push(horizontal_space())
                        .push(panel.width(extend_panel).height(self.panel.size as f32))
                        .align_x(Horizontal::Center)
                        .into(),
                    (Position::Bottom, Position::Bottom) => column
                        .push(vertical_space())
                        .push(horizontal_space())
                        .push(panel.width(extend_panel).height(self.panel.size as f32))
                        .push(dock.width(extend_dock))
                        .align_x(Horizontal::Center)
                        .into(),
                    (Position::Bottom, Position::Left) => row
                        .push(dock.height(extend_dock))
                        .push(
                            widget::column()
                                .push(vertical_space())
                                .push(horizontal_space())
                                .push(panel.width(extend_panel).height(self.panel.size as f32))
                                .align_x(Horizontal::Center),
                        )
                        .align_y(Vertical::Center)
                        .into(),
                    (Position::Bottom, Position::Right) => row
                        .push_maybe(if self.panel.extend {
                            None
                        } else {
                            Some(horizontal_space())
                        })
                        .push(
                            widget::column()
                                .push(vertical_space())
                                .push(panel.width(extend_panel).height(self.panel.size as f32)),
                        )
                        .push_maybe(if self.panel.extend {
                            None
                        } else {
                            Some(horizontal_space())
                        })
                        .push(dock.height(extend_dock))
                        .align_y(Vertical::Center)
                        .into(),
                    (Position::Left, Position::Top) => row
                        .push(panel.width(self.panel.size as f32).height(extend_panel))
                        .push_maybe(if self.dock.extend {
                            None
                        } else {
                            Some(horizontal_space())
                        })
                        .push(
                            widget::column()
                                .push(dock.width(extend_dock))
                                .push(vertical_space())
                                .align_x(Horizontal::Center),
                        )
                        .push_maybe(if self.dock.extend {
                            None
                        } else {
                            Some(horizontal_space())
                        })
                        .align_y(Vertical::Center)
                        .into(),
                    (Position::Left, Position::Bottom) => row
                        .push(panel.width(self.panel.size as f32).height(extend_panel))
                        .push_maybe(if self.dock.extend {
                            None
                        } else {
                            Some(horizontal_space())
                        })
                        .push(
                            widget::column()
                                .push(vertical_space())
                                .push(horizontal_space())
                                .push(dock.width(extend_dock))
                                .align_x(Horizontal::Center),
                        )
                        .push_maybe(if self.dock.extend {
                            None
                        } else {
                            Some(horizontal_space())
                        })
                        .align_y(Vertical::Center)
                        .into(),
                    (Position::Left, Position::Left) => row
                        .push(panel.width(self.panel.size as f32).height(extend_panel))
                        .push(dock.height(extend_dock))
                        .push(horizontal_space())
                        .align_y(Vertical::Center)
                        .into(),
                    (Position::Left, Position::Right) => row
                        .push(panel.width(self.panel.size as f32).height(extend_panel))
                        .push(horizontal_space())
                        .push(dock.height(extend_dock))
                        .align_y(Vertical::Center)
                        .into(),
                    (Position::Right, Position::Top) => row
                        .push_maybe(if self.dock.extend {
                            None
                        } else {
                            Some(horizontal_space())
                        })
                        .push(
                            widget::column()
                                .push(dock.width(extend_dock))
                                .push(vertical_space())
                                .align_x(Horizontal::Center),
                        )
                        .push_maybe(if self.dock.extend {
                            None
                        } else {
                            Some(horizontal_space())
                        })
                        .push(panel.width(self.panel.size as f32).height(extend_panel))
                        .align_y(Vertical::Center)
                        .into(),
                    (Position::Right, Position::Bottom) => row
                        .push_maybe(if self.dock.extend {
                            None
                        } else {
                            Some(horizontal_space())
                        })
                        .push(
                            widget::column()
                                .push(vertical_space())
                                .push(dock.width(extend_dock))
                                .align_x(Horizontal::Center),
                        )
                        .push_maybe(if self.dock.extend {
                            None
                        } else {
                            Some(horizontal_space())
                        })
                        .push(panel.width(self.panel.size as f32).height(extend_panel))
                        .align_y(Vertical::Center)
                        .into(),
                    (Position::Right, Position::Left) => row
                        .push(dock.height(extend_dock))
                        .push(horizontal_space())
                        .push(panel.width(self.panel.size as f32).height(extend_panel))
                        .align_y(Vertical::Center)
                        .into(),
                    (Position::Right, Position::Right) => row
                        .push(horizontal_space())
                        .push(dock.height(extend_dock))
                        .push(panel.width(self.panel.size as f32).height(extend_panel))
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

    pub fn square<'a, Message: Clone + 'a>(size: f32) -> widget::Container<'a, Message, Theme> {
        widget::container(widget::text(""))
            .width(Length::Fixed(size))
            .height(Length::Fixed(size))
            .class(cosmic::style::Container::Secondary)
    }
}

impl PanelProperties {
    pub fn new(position: Position, extend: bool, hidden: bool, size: usize) -> Self {
        Self {
            position,
            extend,
            hidden,
            size,
        }
    }
}

impl Default for LayoutPreview {
    fn default() -> Self {
        Self {
            panel: PanelProperties::new(Position::Top, true, false, 20),
            dock: PanelProperties::new(Position::Bottom, true, false, 20),
            dock_icons: 6,
            show_window: true,
        }
    }
}

impl ToString for Position {
    fn to_string(&self) -> String {
        match self {
            Position::Top => fl!("top"),
            Position::Bottom => fl!("bottom"),
            Position::Left => fl!("left"),
            Position::Right => fl!("right"),
        }
    }
}
