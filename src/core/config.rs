use super::icons::{IconCache, ICON_CACHE};
use std::sync::Mutex;

pub fn get() -> (cosmic::app::Settings, crate::app::Flags) {
    ICON_CACHE.get_or_init(|| Mutex::new(IconCache::new()));
    init_logger();
    (
        cosmic::app::Settings::default().size_limits(
            cosmic::iced::Limits::NONE
                .min_width(360.0)
                .min_height(180.0),
        ),
        crate::app::Flags {
            config_handler: crate::settings::TweaksSettings::config_handler(),
            config: crate::settings::TweaksSettings::config(),
        },
    )
}

fn init_logger() {
    std::env::set_var("RUST_LOG", "cosmic_ext_tweaks=info");
    pretty_env_logger::init();
}
