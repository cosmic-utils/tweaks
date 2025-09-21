use crate::{app::flags::Flags, localize};

use super::icons::{ICON_CACHE, IconCache};
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

    if std::env::var("RUST_LOG").is_err() {
        unsafe {
            std::env::set_var("RUST_LOG", "warn");
        }
    }

    pretty_env_logger::init();
    crate::app::pages::layouts::Layouts::init()
}
