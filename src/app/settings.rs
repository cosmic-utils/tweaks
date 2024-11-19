use crate::core::localize;

use super::icons::{IconCache, ICON_CACHE};
use std::sync::Mutex;

pub fn settings() -> cosmic::app::Settings {
    cosmic::app::Settings::default().size_limits(
        cosmic::iced::Limits::NONE
            .min_width(360.0)
            .min_height(180.0),
    )
}

pub fn flags() -> crate::app::Flags {
    crate::app::Flags {
        config_handler: crate::app::config::TweaksConfig::config_handler(),
        config: crate::app::config::TweaksConfig::config(),
    }
}

pub fn init() {
    // Icon cache
    ICON_CACHE.get_or_init(|| Mutex::new(IconCache::new()));

    // Localize
    localize::localize();

    // Logger
    std::env::set_var("RUST_LOG", "cosmic_ext_tweaks=info");
    pretty_env_logger::init();
}
