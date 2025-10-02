// SPDX-License-Identifier: GPL-3.0-only

use cosmic::widget::icon;
use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

pub static ICON_CACHE: LazyLock<RwLock<HashMap<IconCacheKey, icon::Handle>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct IconCacheKey {
    name: &'static str,
    size: u16,
}

impl IconCacheKey {
    pub fn new(name: &'static str, size: u16) -> Self {
        Self { name, size }
    }
}

#[macro_export]
macro_rules! icon_handle {
    ($name:literal, $size:expr) => {{
        use $crate::app::core::icons::{ICON_CACHE, IconCacheKey};

        let key = IconCacheKey::new($name, $size);

        if let Some(handle) = ICON_CACHE.read().unwrap().get(&key) {
            handle.clone()
        } else {
            let bytes = include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/res/icons/bundled/",
                $name,
                ".svg"
            ));
            let handle = cosmic::widget::icon::from_svg_bytes(bytes).symbolic(true);

            ICON_CACHE.write().unwrap().insert(key, handle.clone());
            handle
        }
    }};
}

#[macro_export]
macro_rules! icon {
    ($name:literal, $size:expr) => {{
        use $crate::icon_handle;
        cosmic::widget::icon::icon(icon_handle!($name, $size))
    }};
}
#[macro_export]
macro_rules! icon_button {
    ($name:literal, $size:expr) => {{
        use $crate::icon_handle;
        cosmic::widget::button::icon(icon_handle!($name, $size))
    }};
}
