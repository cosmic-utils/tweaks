use cosmic::{
    Element, Task,
    cosmic_config::{self, Config, CosmicConfigEntry},
    widget,
};
use cosmic_panel_config::{AutoHide, CosmicPanelConfig};
use serde::{Deserialize, Serialize};

use crate::fl;

use crate::app::core::icons;
use config::{CosmicPanelButtonConfig, IndividualConfig, Override};

pub mod config;
pub mod size;

#[derive(Debug)]
pub struct Panel {
    pub panel_helper: Option<Config>,
    pub panel_config: Option<CosmicPanelConfig>,
    pub padding: u32,
    pub margin: u16,
    pub spacing: u32,
    pub show_panel: bool,
    pub cosmic_panel_config: CosmicPanel,
    pub cosmic_panel_config_helper: Option<Config>,
    pub cosmic_panel_button_config: CosmicPanelButtonConfig,
    pub cosmic_panel_button_config_helper: Option<Config>,
    pub force_icons: bool,
    panel_size: cosmic_panel_config::PanelSize,
    autohide: AutoHide,
}

#[derive(
    Debug,
    Clone,
    Default,
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    cosmic_config::cosmic_config_derive::CosmicConfigEntry,
)]
pub struct CosmicPanel {
    pub entries: Vec<String>,
}

impl Default for Panel {
    fn default() -> Self {
        let panel_helper = CosmicPanelConfig::cosmic_config("Panel").ok();
        let panel_config = panel_helper.as_ref().and_then(|config_helper| {
            let panel_config = CosmicPanelConfig::get_entry(config_helper).ok()?;
            (panel_config.name == "Panel").then_some(panel_config)
        });
        let (cosmic_panel_config_helper, cosmic_panel_config) =
            match cosmic_config::Config::new("com.system76.CosmicPanel", 1) {
                Ok(config_handler) => {
                    let config = match CosmicPanel::get_entry(&config_handler) {
                        Ok(ok) => ok,
                        Err((errs, config)) => {
                            log::error!("errors loading config: {:?}", errs);
                            config
                        }
                    };
                    (Some(config_handler), config)
                }
                Err(err) => {
                    log::error!("failed to create config handler: {}", err);
                    (None, CosmicPanel::default())
                }
            };

        let (cosmic_panel_button_config_helper, cosmic_panel_button_config) =
            match cosmic_config::Config::new("com.system76.CosmicPanelButton", 1) {
                Ok(config_handler) => {
                    let config = match CosmicPanelButtonConfig::get_entry(&config_handler) {
                        Ok(ok) => ok,
                        Err((errs, config)) => {
                            log::error!(
                                "errors loading config for cosmic panel button: {:?}",
                                errs
                            );
                            config
                        }
                    };
                    (Some(config_handler), config)
                }
                Err(err) => {
                    log::error!(
                        "failed to create config handler for cosmic panel button: {}",
                        err
                    );
                    (None, CosmicPanelButtonConfig::default())
                }
            };

        let padding = panel_config
            .clone()
            .map(|config| config.padding)
            .unwrap_or(0);
        let margin = panel_config
            .clone()
            .map(|config| config.margin)
            .unwrap_or(0);
        let spacing = panel_config
            .clone()
            .map(|config| config.spacing)
            .unwrap_or(0);
        let panel_size = panel_config
            .clone()
            .map(|config| config.size)
            .unwrap_or(cosmic_panel_config::PanelSize::M);
        let show_panel = cosmic_panel_config.entries.iter().any(|e| e == "Panel");
        let force_icons = cosmic_panel_button_config
            .configs
            .iter()
            .find(|(e, _)| *e == "Panel")
            .map(|(_, conf)| {
                conf.force_presentation
                    .as_ref()
                    .is_some_and(|presentation| *presentation == Override::Icon)
            })
            .unwrap_or(false);
        let autohide = panel_config
            .clone()
            .map(|config| config.autohide.unwrap_or(AutoHide::default()))
            .unwrap();

        Self {
            panel_helper,
            panel_config,
            padding,
            margin,
            spacing,
            show_panel,
            cosmic_panel_config,
            cosmic_panel_config_helper,
            cosmic_panel_button_config,
            cosmic_panel_button_config_helper,
            force_icons,
            panel_size,
            autohide,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SetPadding(u32),
    SetMargin(u16),
    SetSpacing(u32),
    ShowPanel(bool),
    ForceIcons(bool),
    SetPanelSize(i32),
    SetWaitTime(u32),
    SetTransitionTime(u32),
    SetHandleSize(u32),
}

impl Panel {
    pub fn view<'a>(&self) -> Element<'a, Message> {
        let spacing = cosmic::theme::spacing();

        widget::scrollable(widget::settings::view_column(vec![
            widget::settings::section()
                .title("Panel")
                .add(
                    widget::settings::item::builder(fl!("show-panel"))
                        .icon(icons::get_icon("eye-outline-symbolic", 18))
                        .toggler(self.show_panel, Message::ShowPanel),
                )
                .add(
                    widget::settings::item::builder(fl!("force-icon-buttons-in-panel"))
                        .icon(icons::get_icon("smile-symbolic", 18))
                        .toggler(self.force_icons, Message::ForceIcons),
                )
                .add(
                    widget::settings::item::builder(fl!("size"))
                        .description(fl!("size-description"))
                        .icon(icons::get_icon("size-vertically-symbolic", 18))
                        .control(
                            widget::row()
                                .push(
                                    widget::slider(
                                        16..=112,
                                        size::to_u32(self.panel_size.clone()) as i32,
                                        Message::SetPanelSize,
                                    )
                                    .step(4)
                                    .breakpoints(&[32, 40, 56, 64, 96]),
                                )
                                .push(size::name(self.panel_size.clone()))
                                .spacing(spacing.space_xxs),
                        ),
                )
                .add(
                    widget::settings::item::builder(fl!("padding"))
                        .description(fl!("padding-description"))
                        .icon(icons::get_icon("resize-mode-symbolic", 18))
                        .control(
                            widget::row()
                                .push(widget::slider(0..=20, self.padding, Message::SetPadding))
                                .push(widget::text::text(format!("{} px", self.padding)))
                                .spacing(spacing.space_xxs),
                        ),
                )
                .add(
                    widget::settings::item::builder(fl!("margin"))
                        .description(fl!("margin-description"))
                        .icon(icons::get_icon("object-layout-symbolic", 18))
                        .control(
                            widget::row()
                                .push(widget::slider(0..=20, self.margin, Message::SetMargin))
                                .push(widget::text::text(format!("{} px", self.margin)))
                                .spacing(spacing.space_xxs),
                        ),
                )
                .add(
                    widget::settings::item::builder(fl!("spacing"))
                        .description(fl!("spacing-description"))
                        .icon(icons::get_icon("size-horizontally-symbolic", 18))
                        .control(
                            widget::row()
                                .push(widget::slider(0..=28, self.spacing, Message::SetSpacing))
                                .push(widget::text::text(format!("{} px", self.spacing)))
                                .spacing(spacing.space_xxs),
                        ),
                )
                .into(),
            widget::settings::section()
                .title(fl!("animation-speed"))
                .add(
                    widget::settings::item::builder(fl!("wait-time"))
                        .description(fl!("wait-time-description"))
                        .icon(icons::get_icon("size-vertically-symbolic", 18))
                        .control(
                            widget::row()
                                .push(
                                    widget::slider(
                                        0..=4000,
                                        self.autohide.wait_time,
                                        Message::SetWaitTime,
                                    )
                                    .breakpoints(&[1000, 2000, 3000])
                                    .step(100u32),
                                )
                                .push(widget::text(format!("{} ms", self.autohide.wait_time)))
                                .spacing(spacing.space_xxs),
                        ),
                )
                .add(
                    widget::settings::item::builder(fl!("transition-time"))
                        .description(fl!("transition-time-description"))
                        .icon(icons::get_icon("size-vertically-symbolic", 18))
                        .control(
                            widget::row()
                                .push(
                                    widget::slider(
                                        0..=4000,
                                        self.autohide.transition_time,
                                        Message::SetTransitionTime,
                                    )
                                    .breakpoints(&[1000, 2000, 3000])
                                    .step(100u32),
                                )
                                .push(widget::text(format!(
                                    "{} ms",
                                    self.autohide.transition_time
                                )))
                                .spacing(spacing.space_xxs),
                        ),
                )
                .add(
                    widget::settings::item::builder(fl!("handle-size"))
                        .description(fl!("handle-size-description"))
                        .icon(icons::get_icon("size-vertically-symbolic", 18))
                        .control(
                            widget::row()
                                .push(
                                    widget::slider(
                                        4..=32,
                                        self.autohide.handle_size,
                                        Message::SetHandleSize,
                                    )
                                    .breakpoints(&[8, 12, 16, 20, 24, 28])
                                    .step(4u32),
                                )
                                .push(widget::text(format!("{} px", self.autohide.handle_size)))
                                .spacing(spacing.space_xxs),
                        ),
                )
                .into(),
        ]))
        .into()
    }

    pub fn update(&mut self, message: Message) -> Task<crate::app::message::Message> {
        let Some(panel_helper) = &self.panel_helper else {
            return cosmic::Task::none();
        };
        let Some(panel_config) = &mut self.panel_config else {
            return cosmic::Task::none();
        };

        match message {
            Message::SetPadding(padding) => {
                self.padding = padding;
                let update = panel_config.set_padding(panel_helper, self.padding);
                if let Err(err) = update {
                    log::error!("Error updating panel padding: {}", err);
                }
            }
            Message::SetMargin(margin) => {
                self.margin = margin;
                let update = panel_config.set_margin(panel_helper, self.margin);
                if let Err(err) = update {
                    log::error!("Error updating panel margin: {}", err);
                }
            }
            Message::SetSpacing(spacing) => {
                self.spacing = spacing;
                let update = panel_config.set_spacing(panel_helper, self.spacing);
                if let Err(err) = update {
                    log::error!("Error updating panel spacing: {}", err);
                }
            }
            Message::ForceIcons(force) => {
                let mut configs = self.cosmic_panel_button_config.configs.clone();
                if let Some(inner_config) = configs.get_mut("Panel") {
                    inner_config.force_presentation =
                        if force { Some(Override::Icon) } else { None };
                } else {
                    configs.insert(
                        "Panel".to_owned(),
                        IndividualConfig {
                            force_presentation: if force { Some(Override::Icon) } else { None },
                        },
                    );
                }

                if let Some(helper) = &self.cosmic_panel_button_config_helper {
                    let update = self.cosmic_panel_button_config.set_configs(helper, configs);
                    if let Err(err) = update {
                        log::error!("Error updating cosmic panel button configs: {}", err);
                    } else {
                        self.force_icons = force;
                    }
                }
            }
            Message::ShowPanel(show) => {
                if show {
                    if !self
                        .cosmic_panel_config
                        .entries
                        .iter()
                        .any(|e| e == "Panel")
                    {
                        let mut entries = self.cosmic_panel_config.entries.clone();
                        entries.push("Panel".to_owned());
                        if let Some(helper) = &self.cosmic_panel_config_helper {
                            let update = self.cosmic_panel_config.set_entries(helper, entries);
                            if let Err(err) = update {
                                log::error!("Error updating cosmic panel entries: {}", err);
                            } else {
                                self.show_panel = false;
                            }
                        }
                    }
                } else if let Some(i) = self
                    .cosmic_panel_config
                    .entries
                    .iter()
                    .position(|e| e == "Panel")
                {
                    let mut entries = self.cosmic_panel_config.entries.clone();
                    entries.remove(i);
                    if let Some(helper) = &self.cosmic_panel_config_helper {
                        let update = self.cosmic_panel_config.set_entries(helper, entries);
                        if let Err(err) = update {
                            log::error!("Error updating cosmic panel entries: {}", err);
                        } else {
                            self.show_panel = true;
                        }
                    }
                }
            }
            Message::SetPanelSize(panel_size) => {
                self.panel_size = cosmic_panel_config::PanelSize::Custom(panel_size as u32);
                let update = panel_config.set_size(panel_helper, self.panel_size.clone());
                if let Err(err) = update {
                    log::error!("Error updating panel spacing: {}", err);
                }
            }
            Message::SetWaitTime(wait_time) => {
                self.autohide.wait_time = wait_time;
                if let Err(err) =
                    panel_config.set_autohide(panel_helper, Some(self.autohide.clone()))
                {
                    log::error!("Error updating panel wait time: {}", err);
                }
            }
            Message::SetTransitionTime(transition_time) => {
                self.autohide.transition_time = transition_time;
                if let Err(err) =
                    panel_config.set_autohide(panel_helper, Some(self.autohide.clone()))
                {
                    log::error!("Error updating panel transition time: {}", err);
                }
            }
            Message::SetHandleSize(handle_size) => {
                self.autohide.handle_size = handle_size;
                if let Err(err) =
                    panel_config.set_autohide(panel_helper, Some(self.autohide.clone()))
                {
                    log::error!("Error updating panel handle size: {}", err);
                }
            }
        }
        Task::none()
    }
}
