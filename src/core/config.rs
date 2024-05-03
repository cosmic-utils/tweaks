use std::sync::Mutex;

use super::icons::{IconCache, ICON_CACHE};

pub fn get() -> cosmic::app::Settings {
    ICON_CACHE.get_or_init(|| Mutex::new(IconCache::new()));
    
    cosmic::app::Settings::default()
}
