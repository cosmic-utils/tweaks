use crate::app::flags::Flags;

use super::icons::{IconCache, ICON_CACHE};
use crate::app::core::localize;
use std::sync::Mutex;

pub fn settings() -> cosmic::app::Settings {
    cosmic::app::Settings::default().size_limits(
        cosmic::iced::Limits::NONE
            .min_width(360.0)
            .min_height(180.0),
    )
}

pub fn flags() -> Flags {
    Flags {
        handler: crate::app::core::config::TweaksConfig::config(),
        config: crate::app::core::config::TweaksConfig::new(),
    }
}

pub fn init() -> Result<(), crate::Error> {
    ICON_CACHE.get_or_init(|| Mutex::new(IconCache::new()));
    localize::localize();
    std::env::set_var("RUST_LOG", "cosmic_ext_tweaks=info");
    pretty_env_logger::init();
    crate::app::pages::layouts::Layouts::init()
}
