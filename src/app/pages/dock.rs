use cosmic::{
    cosmic_config::{Config, CosmicConfigEntry},
    widget, Element, Task,
};
use cosmic_panel_config::CosmicPanelConfig;

use crate::app::core::icons;
use crate::fl;

#[derive(Debug)]
pub struct Dock {
    pub dock_helper: Option<Config>,
    pub dock_config: Option<CosmicPanelConfig>,
    pub padding: u32,
    pub spacing: u32,
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
        let spacing = dock_config
            .clone()
            .map(|config| config.spacing)
            .unwrap_or(0);
        Self {
            dock_helper,
            dock_config,
            padding,
            spacing,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SetPadding(u32),
    SetSpacing(u32),
}

impl Dock {
    pub fn view<'a>(&self) -> Element<'a, Message> {
        let spacing = cosmic::theme::spacing();
        widget::scrollable(
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
                    widget::settings::item::builder(fl!("spacing"))
                        .description(fl!("spacing-description"))
                        .icon(icons::get_icon("size-horizontally-symbolic", 18))
                        .control(
                            widget::row()
                                .push(widget::slider(0..=28, self.spacing, Message::SetSpacing))
                                .push(widget::text::text(format!("{} px", self.spacing)))
                                .spacing(spacing.space_xxs),
                        ),
                ),
        )
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
            Message::SetSpacing(spacing) => {
                self.spacing = spacing;
                let update = dock_config.set_spacing(dock_helper, self.spacing);
                if let Err(err) = update {
                    log::error!("Error updating dock spacing: {}", err);
                }
            }
        }
        Task::none()
    }
}
