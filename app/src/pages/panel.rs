use cosmic::{
    cosmic_config::{self, CosmicConfigEntry},
    iced::Alignment,
    widget, Command, Element,
};
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        config_manager::ConfigManager,
        cosmic_panel_button_config::{IndividualConfig, Override},
        icons,
    },
    fl,
    settings::Tweak,
};

#[derive(Debug)]
pub struct Panel {
    pub config_manager: ConfigManager,
    pub padding: u32,
    pub spacing: u32,
    pub show_panel: bool,
    pub force_icons: bool,
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
        let config_manager = ConfigManager::new();
        let padding = config_manager
            .panel_config
            .clone()
            .map(|config| config.padding)
            .unwrap_or(0);
        let spacing = config_manager
            .panel_config
            .clone()
            .map(|config| config.spacing)
            .unwrap_or(0);
        let show_panel = config_manager
            .cosmic_panel_config
            .entries
            .iter()
            .any(|e| e == "Panel");
        let force_icons = config_manager
            .panel_button_config
            .configs
            .iter()
            .find(|(e, _)| *e == "Panel")
            .map(|(_, conf)| {
                conf.force_presentation
                    .as_ref()
                    .is_some_and(|presentation| *presentation == Override::Icon)
            })
            .unwrap_or(false);
        Self {
            config_manager,
            padding,
            spacing,
            show_panel,
            force_icons,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SetPadding(u32),
    SetSpacing(u32),
    ShowPanel(bool),
    ForceIcons(bool),
    ToggleFavorite(Tweak),
}

impl Panel {
    pub fn view<'a>(&self) -> Element<'a, Message> {
        let spacing = cosmic::theme::active().cosmic().spacing;
        let favorite = |tweak| {
            if self.config_manager.app_config.favorites.contains(&tweak) {
                widget::icon::from_name("starred-symbolic")
            } else {
                widget::icon::from_name("non-starred-symbolic")
            }
        };
        widget::scrollable(
            widget::settings::section()
                .title("Panel")
                .add(
                    widget::settings::item::builder(fl!("show-panel"))
                        .toggler(self.show_panel, Message::ShowPanel)
                        .push(
                            widget::button::icon(favorite(Tweak::PanelShow))
                                .on_press(Message::ToggleFavorite(Tweak::PanelShow)),
                        ),
                )
                .add(
                    widget::settings::item::builder(fl!("force-icon-buttons-in-panel"))
                        .toggler(self.force_icons, Message::ForceIcons)
                        .push(
                            widget::button::icon(favorite(Tweak::PanelForceIcons))
                                .on_press(Message::ToggleFavorite(Tweak::PanelForceIcons)),
                        ),
                )
                .add(
                    widget::settings::item::builder(fl!("padding"))
                        .description(fl!("padding-description"))
                        .icon(icons::get_icon("resize-mode-symbolic", 18))
                        .control(
                            widget::row::with_children(vec![
                                widget::slider(0..=20, self.padding, Message::SetPadding).into(),
                                widget::text::text(format!("{} px", self.padding)).into(),
                                widget::button::icon(favorite(Tweak::PanelPadding))
                                    .on_press(Message::ToggleFavorite(Tweak::PanelPadding))
                                    .into(),
                            ])
                            .align_items(Alignment::Center)
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
                                widget::button::icon(favorite(Tweak::PanelSpacing))
                                    .on_press(Message::ToggleFavorite(Tweak::PanelSpacing))
                                    .into(),
                            ])
                            .align_items(Alignment::Center)
                            .spacing(spacing.space_xxs),
                        ),
                ),
        )
        .into()
    }

    pub fn update(&mut self, message: Message) -> Command<crate::app::Message> {
        // Helper for updating config values efficiently
        macro_rules! config_set {
            ($name: ident, $value: expr) => {
                match &self.config_manager.app_handler {
                    Some(config_handler) => {
                        match paste::paste! { self.config_manager.app_config.[<set_ $name>](config_handler, $value) } {
                            Ok(_) => {}
                            Err(err) => {
                                log::warn!(
                                    "failed to save config {:?}: {}",
                                    stringify!($name),
                                    err
                                );
                            }
                        }
                    }
                    None => {
                        self.config_manager.app_config.$name = $value;
                        log::warn!(
                            "failed to save config {:?}: no config handler",
                            stringify!($name)
                        );
                    }
                }
            };
        }

        let Some(panel_helper) = &mut self.config_manager.panel_handler else {
            return cosmic::Command::none();
        };
        let Some(panel_config) = &mut self.config_manager.panel_config else {
            return cosmic::Command::none();
        };

        match message {
            Message::ToggleFavorite(tweak) => {
                let favorites = self.config_manager.app_config.favorites.clone();
                if favorites.contains(&tweak) {
                    let index = favorites.iter().position(|x| *x == tweak).unwrap();
                    let mut favorites = favorites.clone();
                    favorites.remove(index);
                    config_set!(favorites, favorites);
                } else {
                    let mut favorites = favorites.clone();
                    favorites.push(tweak);
                    config_set!(favorites, favorites);
                }
            }
            Message::SetPadding(padding) => {
                self.padding = padding;
                let update = panel_config.set_padding(panel_helper, self.padding);
                if let Err(err) = update {
                    log::error!("Error updating panel padding: {}", err);
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
                let mut configs = self.config_manager.panel_button_config.configs.clone();
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

                if let Some(helper) = &self.config_manager.panel_button_handler {
                    let update = self
                        .config_manager
                        .panel_button_config
                        .set_configs(helper, configs);
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
                        .config_manager
                        .cosmic_panel_config
                        .entries
                        .iter()
                        .any(|e| e == "Panel")
                    {
                        let mut entries = self.config_manager.cosmic_panel_config.entries.clone();
                        entries.push("Panel".to_owned());
                        if let Some(helper) = &self.config_manager.cosmic_panel_handler {
                            let update = self
                                .config_manager
                                .cosmic_panel_config
                                .set_entries(helper, entries);
                            if let Err(err) = update {
                                log::error!("Error updating cosmic panel entries: {}", err);
                            } else {
                                self.show_panel = false;
                            }
                        }
                    }
                } else if let Some(i) = self
                    .config_manager
                    .cosmic_panel_config
                    .entries
                    .iter()
                    .position(|e| e == "Panel")
                {
                    let mut entries = self.config_manager.cosmic_panel_config.entries.clone();
                    entries.remove(i);
                    if let Some(helper) = &self.config_manager.cosmic_panel_handler {
                        let update = self
                            .config_manager
                            .cosmic_panel_config
                            .set_entries(helper, entries);
                        if let Err(err) = update {
                            log::error!("Error updating cosmic panel entries: {}", err);
                        } else {
                            self.show_panel = true;
                        }
                    }
                }
            }
        }
        Command::none()
    }
}
