use cosmic::{
    cosmic_config::{Config, CosmicConfigEntry},
    widget, Command, Element,
};
use cosmic_panel_config::CosmicPanelConfig;

use crate::{core::icons, fl};

#[derive(Debug)]
pub struct Panel {
    pub panel_helper: Option<Config>,
    pub panel_config: Option<CosmicPanelConfig>,
    pub padding: u32,
    pub spacing: u32,
}

impl Default for Panel {
    fn default() -> Self {
        let panel_helper = CosmicPanelConfig::cosmic_config("Panel").ok();
        let panel_config = panel_helper.as_ref().and_then(|config_helper| {
            let panel_config = CosmicPanelConfig::get_entry(config_helper).ok()?;
            (panel_config.name == "Panel").then_some(panel_config)
        });
        let padding = panel_config
            .clone()
            .map(|config| config.padding)
            .unwrap_or(0);
        let spacing = panel_config
            .clone()
            .map(|config| config.spacing)
            .unwrap_or(0);
        Self {
            panel_helper,
            panel_config,
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

impl Panel {
    pub fn view<'a>(&self) -> Element<'a, Message> {
        let spacing = cosmic::theme::active().cosmic().spacing;

        widget::container(
            widget::settings::view_section("Panel")
                .add(
                    widget::settings::item::builder(fl!("padding"))
                        .description(fl!("padding-description"))
                        .icon(icons::get_icon("resize-mode-symbolic", 18))
                        .control(
                            widget::row::with_children(vec![
                                widget::slider(0..=20, self.padding, Message::SetPadding).into(),
                                widget::text::text(format!("{} px", self.padding)).into(),
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
                                widget::slider(0..=28, self.spacing, Message::SetSpacing).into(),
                                widget::text::text(format!("{} px", self.spacing)).into(),
                            ])
                            .spacing(spacing.space_xxs),
                        ),
                ),
        )
        .into()
    }

    pub fn update(&mut self, message: Message) -> Command<crate::app::Message> {
        let Some(panel_helper) = &mut self.panel_helper else {
            return cosmic::Command::none();
        };
        let Some(panel_config) = &mut self.panel_config else {
            return cosmic::Command::none();
        };

        match message {
            Message::SetPadding(padding) => {
                self.padding = padding;
                let update = panel_config.set_padding(panel_helper, self.padding);
                if let Err(err) = update {
                    eprintln!("Error updating panel padding: {}", err);
                }
            }
            Message::SetSpacing(spacing) => {
                self.spacing = spacing;
                let update = panel_config.set_spacing(panel_helper, self.spacing);
                if let Err(err) = update {
                    eprintln!("Error updating panel spacing: {}", err);
                }
            }
        }
        Command::none()
    }
}
