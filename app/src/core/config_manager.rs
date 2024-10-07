use cosmic::cosmic_config::{Config, CosmicConfigEntry};
use cosmic_panel_config::CosmicPanelConfig;

use crate::{
    core::cosmic_panel_button_config::{IndividualConfig, Override},
    pages::panel::CosmicPanel,
    settings::TweaksSettings,
};

use super::cosmic_panel_button_config::CosmicPanelButtonConfig;

#[derive(Debug)]
pub struct ConfigManager {
    pub app_handler: Option<Config>,
    pub app_config: TweaksSettings,
    pub dock_handler: Option<Config>,
    pub dock_config: Option<CosmicPanelConfig>,
    pub panel_handler: Option<Config>,
    pub panel_config: Option<CosmicPanelConfig>,
    pub cosmic_panel_handler: Option<Config>,
    pub cosmic_panel_config: CosmicPanel,
    pub panel_button_config: CosmicPanelButtonConfig,
    pub panel_button_handler: Option<Config>,
}

pub enum Message {
    SetDockPadding(u32),
    SetDockSpacing(u32),
    SetPanelPadding(u32),
    SetPanelSpacing(u32),
    SetPanelVisibility(bool),
    SetPanelForcedIcons(bool),
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigManager {
    pub fn new() -> Self {
        let (app_config, app_handler) =
            (TweaksSettings::config(), TweaksSettings::config_handler());
        let dock_helper = CosmicPanelConfig::cosmic_config("Dock").ok();
        let dock_config = dock_helper.as_ref().and_then(|config_helper| {
            let panel_config = CosmicPanelConfig::get_entry(config_helper).ok()?;
            (panel_config.name == "Dock").then_some(panel_config)
        });
        let panel_handler = CosmicPanelConfig::cosmic_config("Panel").ok();
        let panel_config = panel_handler.as_ref().and_then(|config_helper| {
            let panel_config = CosmicPanelConfig::get_entry(config_helper).ok()?;
            (panel_config.name == "Panel").then_some(panel_config)
        });
        let (cosmic_panel_handler, cosmic_panel_config) =
            match Config::new("com.system76.CosmicPanel", 1) {
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
            match Config::new("com.system76.CosmicPanelButton", 1) {
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
        Self {
            app_handler,
            app_config,
            dock_handler: dock_helper,
            dock_config,
            panel_config,
            panel_handler,
            cosmic_panel_handler,
            cosmic_panel_config,
            panel_button_config: cosmic_panel_button_config,
            panel_button_handler: cosmic_panel_button_config_helper,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::SetDockPadding(padding) => {
                let Some(dock_handler) = &self.dock_handler else {
                    return;
                };

                let Some(dock_config) = &mut self.dock_config else {
                    return;
                };

                if let Err(err) = dock_config.set_padding(&dock_handler, padding) {
                    log::error!("Error updating panel padding: {}", err);
                }
            }
            Message::SetDockSpacing(spacing) => {
                let Some(dock_handler) = &self.dock_handler else {
                    return;
                };

                let Some(dock_config) = &mut self.dock_config else {
                    return;
                };

                if let Err(err) = dock_config.set_spacing(&dock_handler, spacing) {
                    log::error!("Error updating panel padding: {}", err);
                }
            }
            Message::SetPanelPadding(padding) => {
                let Some(panel_handler) = &self.panel_handler else {
                    return;
                };

                let Some(panel_config) = &mut self.panel_config else {
                    return;
                };

                if let Err(err) = panel_config.set_padding(&panel_handler, padding) {
                    log::error!("Error updating panel padding: {}", err);
                }
            }
            Message::SetPanelSpacing(spacing) => {
                let Some(panel_handler) = &self.panel_handler else {
                    return;
                };

                let Some(panel_config) = &mut self.panel_config else {
                    return;
                };

                if let Err(err) = panel_config.set_spacing(&panel_handler, spacing) {
                    log::error!("Error updating panel padding: {}", err);
                }
            }
            Message::SetPanelVisibility(visible) => {
                if visible {
                    if !self
                        .cosmic_panel_config
                        .entries
                        .iter()
                        .any(|e| e == "Panel")
                    {
                        let mut entries = self.cosmic_panel_config.entries.clone();
                        entries.push("Panel".to_owned());
                        if let Some(helper) = &self.cosmic_panel_handler {
                            let update = self.cosmic_panel_config.set_entries(helper, entries);
                            if let Err(err) = update {
                                log::error!("Error updating cosmic panel entries: {}", err);
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
                    if let Some(helper) = &self.cosmic_panel_handler {
                        let update = self.cosmic_panel_config.set_entries(helper, entries);
                        if let Err(err) = update {
                            log::error!("Error updating cosmic panel entries: {}", err);
                        }
                    }
                }
            }
            Message::SetPanelForcedIcons(forced) => {
                let mut configs = self.panel_button_config.configs.clone();
                if let Some(inner_config) = configs.get_mut("Panel") {
                    inner_config.force_presentation =
                        if forced { Some(Override::Icon) } else { None };
                } else {
                    configs.insert(
                        "Panel".to_owned(),
                        IndividualConfig {
                            force_presentation: if forced { Some(Override::Icon) } else { None },
                        },
                    );
                }

                if let Some(helper) = &self.panel_button_handler {
                    let update = self.panel_button_config.set_configs(helper, configs);
                    if let Err(err) = update {
                        log::error!("Error updating cosmic panel button configs: {}", err);
                    }
                }
            }
        }
    }
}
