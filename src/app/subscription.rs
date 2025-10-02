use std::any::TypeId;

use cosmic::{
    Application,
    cosmic_config::{self, Update},
    cosmic_theme::{self, ThemeMode},
    iced::{Event, Subscription, event, keyboard::Event as KeyEvent},
};

use crate::app::message::{Message, SettingsMessage};
use crate::app::{App, core::config::TweaksConfig};

use crate::app::core::config::CONFIG_VERSION;

use super::Cosmic;

impl Cosmic {
    pub fn subscription() -> cosmic::iced::Subscription<Message> {
        struct ConfigSubscription;
        struct ThemeSubscription;

        let mut subscriptions = Vec::new();

        subscriptions.push(event::listen_with(
            |event, _status, _window_id| match event {
                Event::Keyboard(KeyEvent::KeyPressed { key, modifiers, .. }) => {
                    Some(Message::Key(modifiers, key))
                }
                Event::Keyboard(KeyEvent::ModifiersChanged(modifiers)) => {
                    Some(Message::Modifiers(modifiers))
                }
                _ => None,
            },
        ));

        subscriptions.push(
            cosmic_config::config_subscription(
                TypeId::of::<ConfigSubscription>(),
                App::APP_ID.into(),
                CONFIG_VERSION,
            )
            .map(|update: Update<TweaksConfig>| {
                if !update.errors.is_empty() {
                    log::info!(
                        "errors loading config {:?}: {:?}",
                        update.keys,
                        update.errors
                    );
                }

                Message::Settings(SettingsMessage::ConfigUpdate(update.config))
            }),
        );

        subscriptions.push(
            cosmic_config::config_subscription::<_, cosmic_theme::ThemeMode>(
                TypeId::of::<ThemeSubscription>(),
                cosmic_theme::THEME_MODE_ID.into(),
                cosmic_theme::ThemeMode::version(),
            )
            .map(|update: Update<ThemeMode>| {
                if !update.errors.is_empty() {
                    log::info!(
                        "errors loading theme mode {:?}: {:?}",
                        update.keys,
                        update.errors
                    );
                }
                Message::SystemThemeModeChange(update.config)
            }),
        );

        Subscription::batch(subscriptions)
    }
}
