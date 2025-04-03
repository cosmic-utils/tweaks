use cosmic::{
    cosmic_config::{Config, CosmicConfigEntry}, theme, widget::{self, button, column, container, flex_row, horizontal_space, row, scrollable, settings::{item::builder, section}, slider, spin_button, text::{self, text}, tooltip
    }, Apply, Element, Task
};
use cosmic_panel_config::CosmicPanelConfig;

use crate::{
    app::TweakMessage, 
    core::icons, 
    pages::dock::Dock,
    fl
};

use super::{color_schemes::{config::ColorScheme, preview, ColorSchemes}, layouts::config::Layout, panel::Panel};

#[derive(Debug)]
pub struct ThemePack {
    pub color_schemes: ColorSchemes,
    pub dock: Dock,
    pub panel: Panel,
}

impl Default for ThemePack {
    fn default() -> Self {
        let dock = Dock::default();
        let panel = Panel::default();
        let color_schemes = ColorSchemes::default();

        Self {
            color_schemes,
            dock,
            panel,
        }
    }
}

impl ThemePack {
    pub fn view<'a>(&self) -> Element<'a, TweakMessage> {
        let spacing = theme::active().cosmic().spacing;

        // Main Theme Pack section
        let theme_packs_section: cosmic::iced_widget::Column<'_, TweakMessage, cosmic::Theme> = column::with_children(vec![
            row::with_children(vec![
                text::title3(format!("Theme Packs")).into(),
                horizontal_space().into(),
                tooltip::tooltip(
                    icons::get_handle("document-save-symbolic", 16)
                        .apply(button::icon)
                        .padding(spacing.space_xxs)
                        .on_press(TweakMessage::SaveThemePack)
                        .class(cosmic::style::Button::Standard),
                    text(format!("Save Current Color Scheme and Layout as Theme Pack")),
                    tooltip::Position::Bottom,
                ).into(),
                widget::tooltip::tooltip(
                    icons::get_handle("arrow-into-box-symbolic", 16)
                        .apply(widget::button::icon)
                        .padding(spacing.space_xxs)
                        .on_press(TweakMessage::StartColorSchemeImport)
                        .class(cosmic::style::Button::Standard),
                    widget::text(fl!("import-color-scheme")),
                    tooltip::Position::Bottom,
                )
                .into(),
            ])
            .spacing(spacing.space_xxs)
            .into()
        ]).into();

        // Color scheme section
        let color_scheme_section = column::with_children(vec![
            row::with_children(vec![
                text::title3(fl!("color-schemes")).into(),
                horizontal_space().into(),
                tooltip::tooltip(
                    icons::get_handle("document-save-symbolic", 16)
                        .apply(button::icon)
                        .padding(spacing.space_xxs)
                        .on_press(TweakMessage::SaveCurrentColorScheme)
                        .class(cosmic::style::Button::Standard)
                        ,
                    text(fl!("save-current-color-scheme")),
                    tooltip::Position::Bottom,
                )
                .into(),
                tooltip::tooltip(
                    icons::get_handle("arrow-into-box-symbolic", 16)
                        .apply(button::icon)
                        .padding(spacing.space_xxs)
                        .on_press(TweakMessage::StartColorSchemeImport)
                        .class(cosmic::style::Button::Standard)
                        .into(),
                    text(fl!("import-color-scheme")),
                    tooltip::Position::Bottom,
                )
                .into(),
                tooltip::tooltip(
                    icons::get_handle("search-global-symbolic", 16)
                        .apply(button::icon)
                        .padding(spacing.space_xxs)
                        .on_press(TweakMessage::OpenAvailableThemes)
                        .class(cosmic::style::Button::Standard)
                        .into(),
                    text(fl!("find-color-schemes")),
                    tooltip::Position::Bottom,
                )
                .into(),
            ])
            .spacing(spacing.space_xxs)
            .into(),
            section()
                .title(fl!("installed"))
                .add({
                    let themes: Vec<Element<TweakMessage>> = self.color_schemes
                        .installed
                        .iter()
                        .map(|color_scheme| preview::installed(color_scheme, &self.color_schemes.selected))
                        .collect();

                    flex_row(themes)
                        .row_spacing(spacing.space_xs)
                        .column_spacing(spacing.space_xs)
                        .apply(widget::container)
                        .padding([0, spacing.space_xxs])
                })
                .into(),
        ])
        .spacing(spacing.space_xxs)
        .apply(widget::scrollable)
        .into();

        // Dock section
        let dock_section = column::with_children(vec![
            text::title3(fl!("dock")).into(),
            horizontal_space().into(),
            scrollable(
            widget::settings::section()
                .add(
                    widget::settings::item::builder(fl!("padding"))
                        .description(fl!("padding-description"))
                        .icon(icons::get_icon("resize-mode-symbolic", 18))
                        .control(
                            widget::row::with_children(vec![
                                widget::slider(0..=28, self.dock.padding, TweakMessage::SetDockPadding).into(),
                                widget::text::text(format!("{} px", self.dock.padding)).into(),
                            ])
                            .spacing(spacing.space_xxs),
                        ),
                )
                .add(
                    widget::settings::item::builder(fl!("spacing"))
                        .description(fl!("spacing-description"))
                        .icon(icons::get_icon("size-horizontally-symbolic", 18))
                        .control(
                            widget::row::with_children(vec![
                                widget::slider(0..=28, self.dock.spacing, TweakMessage::SetDockSpacing).into(),
                                widget::text::text(format!("{} px", self.dock.spacing)).into(),
                            ])
                            .spacing(spacing.space_xxs),
                        ),
                ),
            ).into(),
        ])
        .into();

        // Panel section
        let panel_section = column::with_children(vec![
            text::title3(fl!("panel")).into(),
            horizontal_space().into(),
            scrollable(
            widget::settings::section()
                .add(
                    widget::settings::item::builder(fl!("show-panel"))
                        .toggler(self.panel.show_panel, TweakMessage::ShowPanel),
                )
                .add(
                    widget::settings::item::builder(fl!("force-icon-buttons-in-panel"))
                        .toggler(self.panel.force_icons, TweakMessage::ForceIcons),
                )
                .add(
                    widget::settings::item::builder(fl!("padding"))
                        .description(fl!("padding-description"))
                        .icon(icons::get_icon("resize-mode-symbolic", 18))
                        .control(
                            widget::row::with_children(vec![
                                widget::slider(0..=20, self.panel.padding, TweakMessage::SetPanelPadding).into(),
                                widget::text::text(format!("{} px", self.panel.padding)).into(),
                            ])
                            .spacing(spacing.space_xxs),
                        ),
                )
                .add(
                    widget::settings::item::builder(fl!("spacing"))
                        .description(fl!("spacing-description"))
                        .icon(icons::get_icon("size-horizontally-symbolic", 18))
                        .control(
                            widget::row::with_children(vec![
                                widget::slider(0..=28, self.panel.spacing, TweakMessage::SetPanelSpacing).into(),
                                widget::text::text(format!("{} px", self.panel.spacing)).into(),
                            ])
                            .spacing(spacing.space_xxs),
                        ),
                ),
            ).into(),
        ])
        .into();

        Element::from(container(
            column::with_children(vec![
                color_scheme_section,
                horizontal_space().height(spacing.space_xs).into(),
                dock_section,
                horizontal_space().height(spacing.space_xs).into(),
                panel_section,
            ])
        ))
    }

    pub fn update(&mut self, message: TweakMessage) -> Task<crate::app::Message>{
        match message {
            TweakMessage::ApplyThemePack((Layout, ColorSchemes)) => println!("Applying Theme Pack!"),
            TweakMessage::SelectThemePack((Layout, ColorSchemes)) => println!("Selecting Theme Pack!"),
            TweakMessage::SaveThemePack => println!("Saving Theme Pack!"),
            TweakMessage::DeleteThemePack => println!("Deleting Theme Pack!"),
            _ => {}
        }

        Task::none()
    }
}