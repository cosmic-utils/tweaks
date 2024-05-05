use crate::{core::icons, fl};
use cosmic::{
    cosmic_config::{Config, CosmicConfigEntry},
    widget, Command, Element,
};
use cosmic_app_list::config::AppListConfig;
use cosmic_panel_config::CosmicPanelConfig;

#[derive(Debug)]
pub struct Dock {
    pub dock_helper: Option<Config>,
    pub dock_config: Option<CosmicPanelConfig>,
    pub app_list_helper: Option<Config>,
    pub app_list_config: Option<AppListConfig>,
    pub padding: u32,
    pub applet_spacing: u32,
    pub app_list_spacing: u16,
}

impl Default for Dock {
    fn default() -> Self {
        let dock_helper = CosmicPanelConfig::cosmic_config("Dock").ok();
        let dock_config = dock_helper.as_ref().and_then(|config_helper| {
            let panel_config = CosmicPanelConfig::get_entry(config_helper).ok()?;
            (panel_config.name == "Dock").then_some(panel_config)
        });

        let app_list_helper = AppListConfig::cosmic_config();
        let app_list_config = app_list_helper
            .as_ref()
            .and_then(|config_helper| AppListConfig::get_entry(config_helper).ok());
        let padding = dock_config
            .clone()
            .map(|config| config.padding)
            .unwrap_or(0);
        let applet_spacing = dock_config
            .clone()
            .map(|config| config.spacing)
            .unwrap_or(0);
        let app_list_spacing = app_list_config
            .clone()
            .map(|config| config.spacing)
            .unwrap_or(0);
        Self {
            dock_helper,
            dock_config,
            app_list_helper,
            app_list_config,
            padding,
            applet_spacing,
            app_list_spacing,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SetPadding(u32),
    SetAppletSpacing(u32),
    SetAppListSpacing(u16),
}

impl Dock {
    pub fn view<'a>(&self) -> Element<'a, Message> {
        let spacing = cosmic::theme::active().cosmic().spacing;
        widget::scrollable(
            widget::settings::view_section("Dock")
                .add(
                    widget::settings::item::builder(fl!("padding"))
                        .description(fl!("padding-description"))
                        .icon(icons::get_icon("resize-mode-symbolic", 18))
                        .control(
                            widget::row::with_children(vec![
                                widget::slider(0..=28, self.padding, Message::SetPadding).into(),
                                widget::text::text(format!("{} px", self.padding)).into(),
                            ])
                            .spacing(spacing.space_xxs),
                        ),
                )
                .add(
                    widget::settings::item::builder(fl!("applet-spacing"))
                        .description(fl!("applet-spacing-description"))
                        .icon(icons::get_icon("size-horizontally-symbolic", 18))
                        .control(
                            widget::row::with_children(vec![
                                widget::slider(
                                    0..=28,
                                    self.applet_spacing,
                                    Message::SetAppletSpacing,
                                )
                                .into(),
                                widget::text::text(format!("{} px", self.applet_spacing)).into(),
                            ])
                            .spacing(spacing.space_xxs),
                        ),
                )
                .add(
                    widget::settings::item::builder(fl!("app-list-spacing"))
                        .description(fl!("app-list-spacing-description"))
                        .icon(icons::get_icon("size-horizontally-symbolic", 18))
                        .control(
                            widget::row::with_children(vec![
                                widget::slider(
                                    0..=28,
                                    self.app_list_spacing,
                                    Message::SetAppListSpacing,
                                )
                                .into(),
                                widget::text::text(format!("{} px", self.app_list_spacing)).into(),
                            ])
                            .spacing(spacing.space_xxs),
                        ),
                ),
        )
        .into()
    }

    pub fn update(&mut self, message: Message) -> Command<crate::app::Message> {
        let Some(dock_helper) = &mut self.dock_helper else {
            return cosmic::Command::none();
        };
        let Some(dock_config) = &mut self.dock_config else {
            return cosmic::Command::none();
        };

        match message {
            Message::SetPadding(padding) => {
                self.padding = padding;
                let update = dock_config.set_padding(dock_helper, self.padding);
                if let Err(err) = update {
                    eprintln!("Error updating dock padding: {}", err);
                }
            }
            Message::SetAppletSpacing(spacing) => {
                self.applet_spacing = spacing;
                let update = dock_config.set_spacing(dock_helper, self.applet_spacing);
                if let Err(err) = update {
                    eprintln!("Error updating dock spacing: {}", err);
                }
            }
            Message::SetAppListSpacing(spacing) => {
                let Some(app_list_helper) = &mut self.app_list_helper else {
                    return cosmic::Command::none();
                };
                let Some(app_list_config) = &mut self.app_list_config else {
                    return cosmic::Command::none();
                };

                self.applet_spacing = spacing as u32;
                let update = app_list_config.set_spacing(app_list_helper, self.app_list_spacing);
                if let Err(err) = update {
                    eprintln!("Error updating dock spacing: {}", err);
                }
            }
        }
        Command::none()
    }
}
