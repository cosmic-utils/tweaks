use cosmic::{
    cosmic_config::{Config, CosmicConfigEntry},
    widget, Element,
};

use self::config::ColorScheme;

mod config;

pub struct ColorSchemes {
    config_helper: Option<Config>,
    config: Option<ColorScheme>,
}

impl ColorSchemes {
    pub fn new() -> Self {
        let config_helper = ColorScheme::config().ok();
        let config = config_helper.as_ref().and_then(|config_helper| {
            let config = ColorScheme::get_entry(config_helper).ok()?;
            Some(config)
        });
        Self {
            config_helper,
            config,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ImportColorScheme(String),
    SetColorScheme(ColorScheme),
    DeleteColorScheme(ColorScheme),
}

pub fn view<'a>() -> Element<'a, Message> {
    widget::container(widget::text::title1("Color schemes")).into()
}
