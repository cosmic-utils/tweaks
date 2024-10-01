use super::icons::{IconCache, ICON_CACHE};
use std::sync::Mutex;

pub fn get() -> cosmic::app::Settings {
    ICON_CACHE.get_or_init(|| Mutex::new(IconCache::new()));
    init_logger();
    cosmic::app::Settings::default()
}

fn init_logger() {
    std::env::set_var("RUST_LOG", "cosmic_ext_tweaks=info");
    pretty_env_logger::init();
}
