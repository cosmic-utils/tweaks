use cosmic::{
    cosmic_config::{Config, CosmicConfigEntry},
    widget, Command, Element,
};
use cosmic_panel_config::CosmicPanelConfig;

use crate::{core::icons, fl};

#[derive(Debug)]
pub struct Dock {
    pub dock_helper: Option<Config>,
    pub dock_config: Option<CosmicPanelConfig>,
    pub padding: u32,
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
            .and_then(|config| Some(config.padding))
            .unwrap_or(0);
        Self {
            dock_helper,
            dock_config,
            padding,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SetPadding(u32),
}

impl Dock {
    pub fn view<'a>(&self) -> Element<'a, Message> {
        widget::container(
            widget::settings::view_section("Dock").add(
                widget::settings::item::builder(fl!("padding"))
                    .description(fl!("padding-description"))
                    .icon(icons::get_icon("resize-mode-symbolic", 18))
                    .control(widget::slider(0..=28, self.padding, Message::SetPadding)),
            ),
        )
        .into()
    }

    pub fn update(&mut self, message: Message) -> Command<crate::app::Message> {
        match message {
            Message::SetPadding(padding) => {
                self.padding = padding;
                let Some(dock_helper) = &mut self.dock_helper else {
                    return cosmic::Command::none();
                };
                let Some(dock_config) = &mut self.dock_config else {
                    return cosmic::Command::none();
                };
                let update = dock_config.set_padding(dock_helper, self.padding);
                if let Err(err) = update {
                    eprintln!("Error updating dock padding: {}", err);
                }
            }
        }
        Command::none()
    }
}
