// SPDX-License-Identifier: GPL-3.0-only

use cosmic::widget::icon;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub(crate) static ICON_CACHE: OnceLock<Mutex<IconCache>> = OnceLock::new();

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct IconCacheKey {
    name: &'static str,
    size: u16,
}

pub struct IconCache {
    cache: HashMap<IconCacheKey, icon::Handle>,
}

impl IconCache {
    pub fn new() -> Self {
        let mut cache = HashMap::new();

        macro_rules! bundle {
            ($name:expr, $size:expr) => {
                let data: &'static [u8] =
                    include_bytes!(concat!("../../../res/icons/bundled/", $name, ".svg"));
                cache.insert(
                    IconCacheKey {
                        name: $name,
                        size: $size,
                    },
                    icon::from_svg_bytes(data).symbolic(true),
                );
            };
        }

        // Menu items
        bundle!("cross-small-square-filled-symbolic", 14);
        bundle!("edit-symbolic", 14);
        bundle!("face-smile-big-symbolic", 14);
        bundle!("plus-square-filled-symbolic", 14);
        bundle!("settings-symbolic", 14);
        bundle!("tabs-stack-symbolic", 14);
        bundle!("info-outline-symbolic", 14);
        bundle!("keyboard-symbolic", 18);
        bundle!("size-vertically-symbolic", 18);
        bundle!("smile-symbolic", 18);
        bundle!("eye-outline-symbolic", 18);

        bundle!("size-horizontally-symbolic", 18);
        bundle!("dock-bottom-symbolic", 18);
        bundle!("dock-top-symbolic", 18);
        bundle!("dark-mode-symbolic", 18);
        bundle!("resize-mode-symbolic", 18);
        bundle!("view-coverflow-symbolic", 18);
        bundle!("snapshots-symbolic", 18);
        bundle!("checkmark-symbolic", 16);
        bundle!("recycling-bin-symbolic", 16);
        bundle!("arrow-into-box-symbolic", 16);
        bundle!("document-save-symbolic", 16);
        bundle!("search-global-symbolic", 16);
        bundle!("list-add-symbolic", 16);
        bundle!("symbolic-link-symbolic", 14);
        bundle!("user-trash-symbolic", 14);
        bundle!("selection-mode-symbolic", 14);
        bundle!("folder-download-symbolic", 14);
        bundle!("arrow-circular-bottom-right-symbolic", 14);

        Self { cache }
    }

    pub fn get(&mut self, name: &'static str, size: u16) -> icon::Icon {
        let handle = self
            .cache
            .entry(IconCacheKey { name, size })
            .or_insert_with(|| icon::from_name(name).size(size).handle())
            .clone();
        icon::icon(handle).size(size)
    }

    pub fn get_handle(&mut self, name: &'static str, size: u16) -> icon::Handle {
        let handle = self
            .cache
            .entry(IconCacheKey { name, size })
            .or_insert_with(|| icon::from_name(name).size(size).handle())
            .clone();
        handle
    }
}

pub fn get_icon(name: &'static str, size: u16) -> icon::Icon {
    let mut icon_cache = ICON_CACHE.get().unwrap().lock().unwrap();
    icon_cache.get(name, size)
}

pub fn get_handle(name: &'static str, size: u16) -> icon::Handle {
    let mut icon_cache = ICON_CACHE.get().unwrap().lock().unwrap();
    icon_cache.get_handle(name, size)
}
