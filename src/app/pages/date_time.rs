use cosmic::{
    Element, Task,
    cosmic_config::{self, ConfigGet, ConfigSet},
    widget::{self, settings},
};

use crate::{app, fl};

const CONF_NAME: &str = "com.system76.CosmicAppletTime";
const CONF_VERS: u64 = 1;
const CONF_STRF: &str = "format_strftime";

pub struct DateTime {
    config: Option<cosmic_config::Config>,
    /// Experimental strftime formatting.
    format_strftime: String,
}

impl Default for DateTime {
    fn default() -> Self {
        let config = cosmic_config::Config::new(CONF_NAME, CONF_VERS)
            .inspect_err(|err| error!("Failed creating config handler for {CONF_NAME}: {err}"))
            .ok();
        let format_strftime = config
            .as_ref()
            .and_then(|config| config.get(CONF_STRF).ok())
            .unwrap_or_default();

        Self {
            config,
            format_strftime,
        }
    }
}

impl DateTime {
    pub fn view(&self) -> Element<'_, Message> {
        settings::section()
            .title(fl!("time-format"))
            .add(
                settings::item::builder(fl!("time-format", "format-strftime")).control(
                    widget::text_input("", &self.format_strftime).on_input(Message::Strftime),
                ),
            )
            .into()
    }

    pub fn update(&mut self, message: Message) -> Task<app::message::Message> {
        match message {
            Message::Strftime(format) => {
                self.format_strftime = format;

                if let Some(config) = self.config.as_ref()
                    && let Err(e) = config.set(CONF_STRF, &self.format_strftime)
                {
                    warn!("Error saving {CONF_NAME}/{CONF_STRF} - {e}");
                };
            }
        }

        Task::none()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Strftime(String),
}
