use crate::{
    app::{
        core::{
            grid::GridMetrics,
            style::{destructive_button, link_button, standard_button},
        },
        pages::{
            ColorSchemes,
            color_schemes::{ColorScheme, ColorSchemeKey, Message, SortBy, Status, Tab},
        },
    },
    icon_handle,
};
use crate::{fl, icon};
use cosmic::{
    self, Apply, Element,
    iced::{Alignment, Length},
    iced_widget::pick_list,
    widget::search_input,
};
use cosmic::{
    iced::alignment::Vertical,
    widget::{
        button, column, container, grid, horizontal_space, mouse_area, responsive, row, scrollable,
        segmented_button, settings, text, toggler, tooltip,
    },
};

impl ColorSchemes {
    pub fn header_end(&self) -> Vec<Element<'_, Message>> {
        let mut v = vec![];

        v.push(
            search_input(fl!("search"), &self.query)
                .on_input(Message::Query)
                .width(200)
                .into(),
        );

        v.push(
            pick_list(
                [
                    SortBy::Az,
                    SortBy::MostDownloaded,
                    SortBy::LastModified,
                    SortBy::Author,
                ],
                Some(&self.sort_by),
                Message::SortBy,
            )
            .into(),
        );

        v
    }

    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        let spacing = cosmic::theme::spacing();
        let active_tab = self.model.active_data::<Tab>().unwrap();
        let tabs = segmented_button::horizontal(&self.model)
            .padding(spacing.space_xxxs)
            .button_alignment(cosmic::iced::Alignment::Center)
            .on_activate(Message::TabSelected);
        let active_tab = match active_tab {
            Tab::Installed => settings::section().add(self.installed_themes()),
            Tab::Available => settings::section().add(self.available_themes()),
        };

        column()
            .push(tabs)
            .push(active_tab)
            .spacing(spacing.space_xxs)
            .into()
    }

    fn installed_themes<'a>(&'a self) -> Element<'a, Message> {
        if self.installed.is_empty() {
            text(fl!("no-color-schemes-installed")).into()
        } else {
            responsive(move |size| {
                let spacing = cosmic::theme::spacing();

                let GridMetrics {
                    cols,
                    item_width,
                    column_spacing,
                } = GridMetrics::custom(&spacing, size.width as usize);

                let mut grid = grid();
                let mut col = 0;
                for (key, color_scheme) in self.values() {
                    if col >= cols {
                        grid = grid.insert_row();
                        col = 0;
                    }
                    grid = grid.push(
                        self.installed(
                            key,
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

                scrollable(
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
                    text(fl!("no-color-schemes-found")).into()
                } else {
                    responsive(move |size| {
                        let spacing = cosmic::theme::spacing();

                        let GridMetrics {
                            cols,
                            item_width,
                            column_spacing,
                        } = GridMetrics::custom(&spacing, size.width as usize);

                        let mut grid = grid();
                        let mut col = 0;
                        for (key, color_scheme) in self.values() {
                            if col >= cols {
                                grid = grid.insert_row();
                                col = 0;
                            }

                            grid = grid.push(
                                self.available(
                                    key,
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

                        scrollable(
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
            Status::Loading => text(fl!("loading")).into(),
        }
    }

    pub fn footer(&self) -> Option<Element<'_, Message>> {
        let spacing = cosmic::theme::spacing();

        let dark_mode = row()
            .align_y(Vertical::Center)
            .push(icon!("dark-mode-2-symbolic", 48))
            .push(horizontal_space().width(10))
            .push(toggler(self.theme_mode.is_dark).on_toggle(Message::ToggleDarkMode));

        match self.model.active_data::<Tab>().unwrap() {
            Tab::Installed => Some(
                row()
                    .align_y(Vertical::Center)
                    .push(dark_mode)
                    .push(horizontal_space())
                    .push(
                        button::standard(fl!("save-current-color-scheme"))
                            .trailing_icon(icon_handle!("arrow-into-box-symbolic", 16))
                            .spacing(spacing.space_xs)
                            .on_press(Message::SaveCurrentColorScheme(None)),
                    )
                    .push(
                        button::standard(fl!("import-color-scheme"))
                            .trailing_icon(icon_handle!("document-save-symbolic", 16))
                            .spacing(spacing.space_xs)
                            .on_press(Message::StartImport),
                    )
                    .spacing(spacing.space_xxs)
                    .apply(container)
                    .class(cosmic::style::Container::Card)
                    .padding(spacing.space_xxs)
                    .into(),
            ),
            Tab::Available => Some(
                row()
                    .align_y(Vertical::Center)
                    .push(dark_mode)
                    .push(horizontal_space())
                    .push(match self.status {
                        Status::Idle => button::standard(fl!("refresh"))
                            .on_press(Message::FetchAvailableColorSchemes),
                        Status::Loading => button::standard(fl!("loading")),
                    })
                    .push(
                        button::text(fl!("revert-old-color-scheme")).on_press_maybe(
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
                    .apply(container)
                    .class(cosmic::style::Container::Card)
                    .padding(spacing.space_xxs)
                    .into(),
            ),
        }
    }

    fn installed<'a>(
        &self,
        key: ColorSchemeKey,
        color_scheme: &'a ColorScheme,
        _selected: bool,
        spacing: &cosmic::cosmic_theme::Spacing,
        item_width: usize,
    ) -> Element<'a, super::Message> {
        let theme = &color_scheme.theme;

        mouse_area(
            column()
                .push(
                    row()
                        .push(horizontal_space())
                        .push(text(&color_scheme.name))
                        .push(horizontal_space())
                        .padding(spacing.space_xxs),
                )
                .push(
                    row()
                        .push(
                            container(text(fl!("navigation")))
                                .padding(spacing.space_xxs)
                                .width(90.0)
                                .height(Length::Fill)
                                .class(crate::app::core::style::card(theme)),
                        )
                        .push(horizontal_space())
                        .push(tooltip::tooltip(
                            icon_handle!("symbolic-link-symbolic", 14)
                                .apply(button::icon)
                                .class(link_button(theme.clone()))
                                .padding(spacing.space_xxs)
                                .on_press_maybe(color_scheme.path.clone().map(Message::OpenFolder)),
                            text(fl!("open-containing-folder")),
                            tooltip::Position::Bottom,
                        ))
                        .push(tooltip::tooltip(
                            icon_handle!("user-trash-symbolic", 14)
                                .apply(button::icon)
                                .class(destructive_button(theme.clone()))
                                .padding(spacing.space_xxs)
                                .on_press(super::Message::DeleteColorScheme(key.clone())),
                            text(fl!("delete-color-scheme")),
                            tooltip::Position::Bottom,
                        ))
                        .align_y(Alignment::End)
                        .spacing(spacing.space_xxs)
                        .padding([0, spacing.space_xxs, spacing.space_xxs, spacing.space_xxs]),
                )
                .width(item_width as f32)
                .height(160.)
                .apply(container)
                .class(crate::app::core::style::background(theme)),
        )
        .on_press(Message::SetColorScheme(key))
        .into()
    }

    fn available<'a>(
        &self,
        key: ColorSchemeKey,
        color_scheme: &'a ColorScheme,
        _selected: bool,
        spacing: &cosmic::cosmic_theme::Spacing,
        item_width: usize,
    ) -> Element<'a, Message> {
        let theme = &color_scheme.theme;

        mouse_area(
            column()
                .push(
                    column()
                        .push(text(&color_scheme.name))
                        .push_maybe(
                            color_scheme
                                .author
                                .as_ref()
                                .map(|author| text::caption(fl!("by", author = author))),
                        )
                        .width(Length::Fill)
                        .align_x(Alignment::Center)
                        .padding([spacing.space_xxs, spacing.space_none]),
                )
                .push(
                    row()
                        .push(
                            container(text(fl!("navigation")))
                                .padding(spacing.space_xxs)
                                .width(90.0)
                                .height(Length::Fill)
                                .class(crate::app::core::style::card(theme)),
                        )
                        .push(horizontal_space())
                        .push(tooltip(
                            icon_handle!("symbolic-link-symbolic", 14)
                                .apply(button::icon)
                                .class(link_button(theme.clone()))
                                .padding(spacing.space_xxs)
                                .on_press_maybe(color_scheme.link.clone().map(Message::OpenLink)),
                            text(fl!("open-link")),
                            tooltip::Position::Bottom,
                        ))
                        .push(tooltip(
                            icon_handle!("folder-download-symbolic", 14)
                                .apply(button::icon)
                                .class(standard_button(theme.clone()))
                                .padding(spacing.space_xxs)
                                .on_press_maybe(
                                    (!self.installed.contains_key(&color_scheme.name))
                                        .then_some(Message::InstallColorScheme(key.clone())),
                                ),
                            text(fl!("install-color-scheme")),
                            tooltip::Position::Bottom,
                        ))
                        .align_y(Alignment::End)
                        .spacing(spacing.space_xxs)
                        .padding([0, spacing.space_xxs, spacing.space_xxs, spacing.space_xxs]),
                )
                .width(item_width as f32)
                .height(160.)
                .apply(container)
                .class(crate::app::core::style::background(theme)),
        )
        .on_press(Message::SetColorSchemeWithRollBack(key))
        .into()
    }
}
