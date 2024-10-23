// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{
    app::{Command, Core},
    applet::{self, padded_control},
    iced::{
        wayland::popup::{destroy_popup, get_popup},
        window::Id,
        Alignment, Length, Limits,
    },
    iced_style::application,
    widget, Application, Element, Theme,
};

use cosmic_ext_tweaks::{
    core::{
        config_manager::{self, ConfigManager},
        cosmic_panel_button_config::Override,
    },
    settings::Tweak,
};

use crate::fl;

#[derive(Default)]
pub struct TweaksApplet {
    core: Core,
    popup: Option<Id>,
    config_manager: ConfigManager,
    dock_padding: u32,
    dock_spacing: u32,
    panel_padding: u32,
    panel_spacing: u32,
    panel_show: bool,
    panel_force_icons: bool,
}

impl TweaksApplet {
    fn tweak_view(&self, tweak: Tweak) -> Option<Element<Message>> {
        if !self.config_manager.app_config.favorites.contains(&tweak) {
            return None;
        }

        let spacing = cosmic::theme::active().cosmic().spacing;

        let view = match tweak {
            Tweak::DockPadding => padded_control(
                widget::row::with_children(vec![
                    widget::text(fl!("dock-padding")).into(),
                    widget::horizontal_space(Length::Fill).into(),
                    widget::slider(0..=28, self.dock_padding, Message::SetDockPadding).into(),
                    widget::text::text(format!("{} px", self.dock_padding)).into(),
                ])
                .spacing(spacing.space_xxs),
            ),
            Tweak::DockSpacing => padded_control(
                widget::row::with_children(vec![
                    widget::text(fl!("dock-spacing")).into(),
                    widget::horizontal_space(Length::Fill).into(),
                    widget::slider(0..=28, self.dock_spacing, Message::SetDockSpacing).into(),
                    widget::text::text(format!("{} px", self.dock_spacing)).into(),
                ])
                .spacing(spacing.space_xxs),
            ),
            Tweak::PanelPadding => padded_control(
                widget::row::with_children(vec![
                    widget::text(fl!("panel-padding")).into(),
                    widget::horizontal_space(Length::Fill).into(),
                    widget::slider(0..=28, self.panel_padding, Message::SetPanelPadding).into(),
                    widget::text::text(format!("{} px", self.panel_padding)).into(),
                ])
                .spacing(spacing.space_xxs),
            ),
            Tweak::PanelSpacing => padded_control(
                widget::row::with_children(vec![
                    widget::text(fl!("panel-spacing")).into(),
                    widget::horizontal_space(Length::Fill).into(),
                    widget::slider(0..=28, self.panel_spacing, Message::SetPanelSpacing).into(),
                    widget::text::text(format!("{} px", self.panel_spacing)).into(),
                ])
                .spacing(spacing.space_xxs),
            ),
            Tweak::PanelShow => padded_control(widget::row::with_children(vec![
                widget::text(fl!("panel-show")).into(),
                widget::horizontal_space(Length::Fill).into(),
                widget::toggler(None, self.panel_show, Message::SetPanelVisibility).into(),
            ])),
            Tweak::PanelForceIcons => padded_control(widget::row::with_children(vec![
                widget::text(fl!("panel-force-icons")).into(),
                widget::horizontal_space(Length::Fill).into(),
                widget::toggler(None, self.panel_force_icons, Message::SetPanelForcedIcons).into(),
            ])),
        };

        Some(view.into())
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    PopupClosed(Id),
    OpenTweaks,
    SetDockPadding(u32),
    SetDockSpacing(u32),
    SetPanelPadding(u32),
    SetPanelSpacing(u32),
    SetPanelVisibility(bool),
    SetPanelForcedIcons(bool),
    ReloadConfig,
    ToggleReloadedPopup,
}

impl Application for TweaksApplet {
    type Executor = cosmic::executor::Default;

    type Flags = ();

    type Message = Message;

    const APP_ID: &'static str = "dev.edfloreshz.CosmicTweaks.Applet";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Command<Self::Message>) {
        log::info!("Starting Tweaks Applet...");

        let config_manager = ConfigManager::new();

        let dock_config = config_manager.dock_config.clone();
        let panel_config = config_manager.panel_config.clone();
        let cosmic_panel_config = config_manager.cosmic_panel_config.clone();
        let panel_force_icons = config_manager
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

        let app = TweaksApplet {
            config_manager,
            core,
            popup: None,
            dock_padding: dock_config.clone().map(|s| s.padding).unwrap_or(0),
            dock_spacing: dock_config.clone().map(|s| s.spacing).unwrap_or(0),
            panel_padding: panel_config.clone().map(|s| s.padding).unwrap_or(0),
            panel_spacing: panel_config.clone().map(|s| s.spacing).unwrap_or(0),
            panel_show: cosmic_panel_config.entries.iter().any(|e| e == "Panel"),
            panel_force_icons,
        };

        (app, Command::none())
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn view(&self) -> Element<Self::Message> {
        self.core
            .applet
            .icon_button("utilities-tweak-tool-symbolic")
            .on_press(Message::ToggleReloadedPopup)
            .into()
    }

    fn view_window(&self, _id: Id) -> Element<Self::Message> {
        let spacing = cosmic::theme::active().cosmic().spacing;
        let open_app =
            applet::menu_button(widget::text(fl!("open-app"))).on_press(Message::OpenTweaks);

        let mut content =
            widget::column::with_capacity(self.config_manager.app_config.favorites.len())
                .align_items(Alignment::Start)
                .padding([8, 0]);

        for tweak in &self.config_manager.app_config.favorites {
            if let Some(tweak) = self.tweak_view(*tweak) {
                let divider = applet::padded_control(widget::divider::horizontal::default())
                    .padding([spacing.space_xxs, spacing.space_s]);
                content = content.push(tweak);
                content = content.push(divider);
            }
        }

        content = content.push(open_app);
        self.core.applet.popup_container(content).into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::OpenTweaks => {
                let cmd = std::process::Command::new("cosmic-ext-tweaks");
                tokio::spawn(cosmic::process::spawn(cmd));
            }
            Message::ToggleReloadedPopup => {
                return Command::batch(vec![
                    self.update(Message::ReloadConfig),
                    self.update(Message::TogglePopup),
                ])
            }
            Message::TogglePopup => {
                return if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    let new_id = Id::unique();
                    self.popup.replace(new_id);
                    let mut popup_settings =
                        self.core
                            .applet
                            .get_popup_settings(Id::MAIN, new_id, None, None, None);
                    popup_settings.positioner.size_limits = Limits::NONE
                        .max_width(372.0)
                        .min_width(300.0)
                        .min_height(200.0)
                        .max_height(1080.0);
                    get_popup(popup_settings)
                }
            }
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
            Message::SetDockPadding(padding) => {
                self.dock_padding = padding;
                self.config_manager
                    .update(config_manager::Message::SetDockPadding(padding))
            }
            Message::SetDockSpacing(spacing) => {
                self.dock_spacing = spacing;
                self.config_manager
                    .update(config_manager::Message::SetDockSpacing(spacing))
            }
            Message::SetPanelPadding(padding) => {
                self.panel_padding = padding;
                self.config_manager
                    .update(config_manager::Message::SetPanelPadding(padding))
            }
            Message::SetPanelSpacing(spacing) => {
                self.panel_spacing = spacing;
                self.config_manager
                    .update(config_manager::Message::SetPanelSpacing(spacing))
            }
            Message::SetPanelVisibility(visible) => {
                self.panel_show = visible;
                self.config_manager
                    .update(config_manager::Message::SetPanelVisibility(visible))
            }
            Message::SetPanelForcedIcons(forced) => {
                self.panel_force_icons = forced;
                self.config_manager
                    .update(config_manager::Message::SetPanelForcedIcons(forced))
            }
            Message::ReloadConfig => self.config_manager = ConfigManager::new(),
        }
        Command::none()
    }

    fn style(&self) -> Option<<Theme as application::StyleSheet>::Style> {
        Some(cosmic::applet::style())
    }
}
