use crate::fl;
use crate::{
    app::{
        core::{
            grid::GridMetrics,
            style::{destructive_button, link_button, standard_button},
        },
        pages::{
            ColorSchemes,
            color_schemes::{ColorScheme, Message, Status, Tab},
        },
    },
    icon_handle,
};
use cosmic::{
    Apply, Element,
    iced::{Alignment, Length},
    widget::{self, button, mouse_area, tooltip},
};

impl ColorSchemes {
    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        let spacing = cosmic::theme::spacing();
        let active_tab = self.model.active_data::<Tab>().unwrap();
        let title = widget::text::title3(fl!("color-schemes"));
        let tabs = widget::segmented_button::horizontal(&self.model)
            .padding(spacing.space_xxxs)
            .button_alignment(cosmic::iced::Alignment::Center)
            .on_activate(Message::TabSelected);
        let active_tab = match active_tab {
            Tab::Installed => widget::settings::section().add(self.installed_themes()),
            Tab::Available => widget::settings::section().add(self.available_themes()),
        };

        widget::column()
            .push(title)
            .push(tabs)
            .push(active_tab)
            .spacing(spacing.space_xxs)
            .into()
    }

    fn installed_themes<'a>(&'a self) -> Element<'a, Message> {
        if self.installed.is_empty() {
            widget::text("No color schemes installed").into()
        } else {
            widget::responsive(move |size| {
                let spacing = cosmic::theme::spacing();

                let GridMetrics {
                    cols,
                    item_width,
                    column_spacing,
                } = GridMetrics::custom(&spacing, size.width as usize);

                let mut grid = widget::grid();
                let mut col = 0;
                for color_scheme in self.installed.values() {
                    if col >= cols {
                        grid = grid.insert_row();
                        col = 0;
                    }
                    grid = grid.push(
                        self.installed(
                            color_scheme,
                            self.config
                                .current_config
                                .as_ref()
                                .map(|c| c.name == color_scheme.name)
                                .unwrap_or(false),
                            &spacing,
                            item_width,
                        ),
                    );
                    col += 1;
                }

                widget::scrollable(
                    grid.column_spacing(column_spacing)
                        .row_spacing(column_spacing),
                )
                .height(Length::Fill)
                .width(Length::Fill)
                .into()
            })
            .into()
        }
    }

    fn available_themes<'a>(&'a self) -> Element<'a, Message> {
        match self.status {
            Status::Idle => {
                if self.available.is_empty() {
                    widget::text("No color schemes found").into()
                } else {
                    widget::responsive(move |size| {
                        let spacing = cosmic::theme::spacing();

                        let GridMetrics {
                            cols,
                            item_width,
                            column_spacing,
                        } = GridMetrics::custom(&spacing, size.width as usize);

                        let mut grid = widget::grid();
                        let mut col = 0;
                        for color_scheme in self.available.iter() {
                            if col >= cols {
                                grid = grid.insert_row();
                                col = 0;
                            }

                            grid = grid.push(self.available(color_scheme, &spacing, item_width));
                            col += 1;
                        }

                        widget::scrollable(
                            grid.column_spacing(column_spacing)
                                .row_spacing(column_spacing),
                        )
                        .height(Length::Fill)
                        .width(Length::Fill)
                        .into()
                    })
                    .into()
                }
            }
            Status::Loading => widget::text(fl!("loading")).into(),
        }
    }

    pub fn footer(&self) -> Option<Element<'_, Message>> {
        let spacing = cosmic::theme::spacing();

        match self.model.active_data::<Tab>().unwrap() {
            Tab::Installed => Some(
                widget::row()
                    .push(widget::horizontal_space())
                    .push(
                        widget::button::standard(fl!("save-current-color-scheme"))
                            .trailing_icon(icon_handle!("arrow-into-box-symbolic", 16))
                            .spacing(spacing.space_xs)
                            .on_press(Message::SaveCurrentColorScheme(None)),
                    )
                    .push(
                        widget::button::standard(fl!("import-color-scheme"))
                            .trailing_icon(icon_handle!("document-save-symbolic", 16))
                            .spacing(spacing.space_xs)
                            .on_press(Message::StartImport),
                    )
                    .spacing(spacing.space_xxs)
                    .apply(widget::container)
                    .class(cosmic::style::Container::Card)
                    .padding(spacing.space_xxs)
                    .into(),
            ),
            Tab::Available => Some(
                widget::row()
                    .push(widget::horizontal_space())
                    .push(match self.status {
                        Status::Idle => widget::button::standard("refresh")
                            .on_press(Message::FetchAvailableColorSchemes),
                        Status::Loading => widget::button::standard(fl!("loading")),
                    })
                    .push(
                        button::text("Revert old theme").on_press_maybe(
                            match (&self.saved_color_theme, &self.config.current_config) {
                                (None, None) => false,
                                (None, Some(_)) => false,
                                (Some(_), None) => true,
                                (Some(old), Some(current)) => old.name != current.name,
                            }
                            .then_some(Message::RevertOldTheme),
                        ),
                    )
                    .spacing(spacing.space_xxs)
                    .apply(widget::container)
                    .class(cosmic::style::Container::Card)
                    .padding(spacing.space_xxs)
                    .into(),
            ),
        }
    }

    fn installed<'a>(
        &self,
        color_scheme: &ColorScheme,
        _selected: bool,
        spacing: &cosmic::cosmic_theme::Spacing,
        item_width: usize,
    ) -> Element<'a, super::Message> {
        let theme = color_scheme.theme.clone().build();
        let color_scheme_name = color_scheme.name.clone();

        mouse_area(
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
                            icon_handle!("symbolic-link-symbolic", 14)
                                .apply(widget::button::icon)
                                .class(link_button(theme.clone()))
                                .padding(spacing.space_xxs)
                                .on_press_maybe(color_scheme.path.clone().map(Message::OpenFolder)),
                            widget::text(fl!("open-containing-folder")),
                            tooltip::Position::Bottom,
                        ))
                        .push(widget::tooltip::tooltip(
                            icon_handle!("user-trash-symbolic", 14)
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
                .class(crate::app::core::style::background(&theme)),
        )
        .on_press(super::Message::SetColorScheme(color_scheme.clone()))
        .into()
    }

    fn available<'a>(
        &self,
        color_scheme: &'a ColorScheme,
        spacing: &cosmic::cosmic_theme::Spacing,
        item_width: usize,
    ) -> Element<'a, Message> {
        let theme = color_scheme.theme.clone().build();

        mouse_area(
            widget::column()
                .push(
                    widget::column()
                        .push(widget::text(&color_scheme.name))
                        .push_maybe(
                            color_scheme
                                .author
                                .as_ref()
                                .map(|author| widget::text::caption(format!("By {}", author))),
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
                            icon_handle!("symbolic-link-symbolic", 14)
                                .apply(widget::button::icon)
                                .class(link_button(theme.clone()))
                                .padding(spacing.space_xxs)
                                .on_press_maybe(color_scheme.link.clone().map(Message::OpenLink)),
                            widget::text(fl!("open-link")),
                            cosmic::widget::tooltip::Position::Bottom,
                        ))
                        .push(widget::tooltip::tooltip(
                            icon_handle!("folder-download-symbolic", 14)
                                .apply(widget::button::icon)
                                .class(standard_button(theme.clone()))
                                .padding(spacing.space_xxs)
                                .on_press_maybe(
                                    (!self.installed.contains_key(&color_scheme.name)).then_some(
                                        Message::InstallColorScheme(color_scheme.clone()),
                                    ),
                                ),
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
                .class(crate::app::core::style::background(&theme)),
        )
        .on_press(super::Message::SetColorSchemeWithRollBack(
            color_scheme.clone(),
        ))
        .into()
    }
}
