use cosmic::{
    Element, Task,
    cosmic_config::{Config, CosmicConfigEntry},
    widget,
};
use cosmic_panel_config::{AutoHide, CosmicPanelConfig};

use crate::app::core::icons;
use crate::fl;

#[derive(Debug)]
pub struct Dock {
    pub dock_helper: Option<Config>,
    pub dock_config: Option<CosmicPanelConfig>,
    pub padding: u32,
    pub margin: u16,
    pub spacing: u32,
    autohide: AutoHide,
}

impl Default for Dock {
    fn default() -> Self {
        let dock_helper = CosmicPanelConfig::cosmic_config("Dock").ok();
        let dock_config = dock_helper.as_ref().and_then(|config_helper| {
            let panel_config = CosmicPanelConfig::get_entry(config_helper).ok()?;
            (panel_config.name == "Dock").then_some(panel_config)
        });
        let padding = dock_config
            .clone()
            .map(|config| config.padding)
            .unwrap_or(0);
        let margin = dock_config.clone().map(|config| config.margin).unwrap_or(0);
        let spacing = dock_config
            .clone()
            .map(|config| config.spacing)
            .unwrap_or(0);
        let autohide = dock_config
            .clone()
            .map(|config| config.autohide.unwrap_or(AutoHide::default()))
            .unwrap_or_default();
        Self {
            dock_helper,
            dock_config,
            padding,
            margin,
            spacing,
            autohide,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SetPadding(u32),
    SetMargin(u16),
    SetSpacing(u32),
    SetWaitTime(u32),
    SetTransitionTime(u32),
    SetHandleSize(u32),
}

impl Dock {
    pub fn view<'a>(&self) -> Element<'a, Message> {
        let spacing = cosmic::theme::spacing();
        widget::scrollable(widget::settings::view_column(vec![
            widget::settings::section()
                .title("Dock")
                .add(
                    widget::settings::item::builder(fl!("padding"))
                        .description(fl!("padding-description"))
                        .icon(icons::get_icon("resize-mode-symbolic", 18))
                        .control(
                            widget::row()
                                .push(widget::slider(0..=28, self.padding, Message::SetPadding))
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
        let Some(dock_helper) = &mut self.dock_helper else {
            return cosmic::Task::none();
        };
        let Some(dock_config) = &mut self.dock_config else {
            return cosmic::Task::none();
        };

        match message {
            Message::SetPadding(padding) => {
                self.padding = padding;
                let update = dock_config.set_padding(dock_helper, self.padding);
                if let Err(err) = update {
                    log::error!("Error updating dock padding: {}", err);
                }
            }
            Message::SetMargin(margin) => {
                self.margin = margin;
                let update = dock_config.set_margin(dock_helper, self.margin);
                if let Err(err) = update {
                    log::error!("Error updating dock margin: {}", err);
                }
            }
            Message::SetSpacing(spacing) => {
                self.spacing = spacing;
                let update = dock_config.set_spacing(dock_helper, self.spacing);
                if let Err(err) = update {
                    log::error!("Error updating dock spacing: {}", err);
                }
            }
            Message::SetWaitTime(wait_time) => {
                self.autohide.wait_time = wait_time;
                if let Err(err) = dock_config.set_autohide(dock_helper, Some(self.autohide.clone()))
                {
                    log::error!("Error updating panel wait time: {}", err);
                }
            }
            Message::SetTransitionTime(transition_time) => {
                self.autohide.transition_time = transition_time;
                if let Err(err) = dock_config.set_autohide(dock_helper, Some(self.autohide.clone()))
                {
                    log::error!("Error updating panel transition time: {}", err);
                }
            }
            Message::SetHandleSize(handle_size) => {
                self.autohide.handle_size = handle_size;
                if let Err(err) = dock_config.set_autohide(dock_helper, Some(self.autohide.clone()))
                {
                    log::error!("Error updating panel handle size: {}", err);
                }
            }
        }
        Task::none()
    }
}
