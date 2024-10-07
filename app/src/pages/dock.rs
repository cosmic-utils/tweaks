use cosmic::{iced::Alignment, widget, Command, Element};

use crate::{
    core::{config_manager::ConfigManager, icons},
    fl,
    settings::Tweak,
};

#[derive(Debug)]
pub struct Dock {
    pub config_manager: ConfigManager,
    pub padding: u32,
    pub spacing: u32,
}

impl Default for Dock {
    fn default() -> Self {
        let config_manager = ConfigManager::new();
        let padding = config_manager
            .dock_config
            .clone()
            .map(|config| config.padding)
            .unwrap_or(0);
        let spacing = config_manager
            .dock_config
            .clone()
            .map(|config| config.spacing)
            .unwrap_or(0);
        Self {
            config_manager,
            padding,
            spacing,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SetPadding(u32),
    SetSpacing(u32),
    ToggleFavorite(Tweak),
}

impl Dock {
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
                .title("Dock")
                .add(
                    widget::settings::item::builder(fl!("padding"))
                        .description(fl!("padding-description"))
                        .icon(icons::get_icon("resize-mode-symbolic", 18))
                        .control(
                            widget::row::with_children(vec![
                                widget::slider(0..=28, self.padding, Message::SetPadding).into(),
                                widget::text::text(format!("{} px", self.padding)).into(),
                                widget::button::icon(favorite(Tweak::DockPadding))
                                    .on_press(Message::ToggleFavorite(Tweak::DockPadding))
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
                                widget::button::icon(favorite(Tweak::DockSpacing))
                                    .on_press(Message::ToggleFavorite(Tweak::DockSpacing))
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

        let Some(dock_helper) = &mut self.config_manager.dock_handler else {
            return cosmic::Command::none();
        };
        let Some(dock_config) = &mut self.config_manager.dock_config else {
            return cosmic::Command::none();
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
        }
        Command::none()
    }
}
