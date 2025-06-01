use cosmic::{
    widget::menu::{self, ItemHeight, ItemWidth},
    Element,
};

use crate::app::action::TweaksAction;
use crate::app::message::Message;
use crate::app::App;

use crate::{core::icons, fl};

use super::Cosmic;

impl Cosmic {
    pub fn header_start(app: &App) -> Vec<Element<Message>> {
        let menu_bar = menu::bar(vec![menu::Tree::with_children(
            menu::root(fl!("view")),
            menu::items(
                &app.cosmic.key_binds,
                vec![
                    menu::Item::Button(
                        fl!("settings"),
                        Some(icons::get_handle("settings-symbolic", 14)),
                        TweaksAction::Settings,
                    ),
                    menu::Item::Divider,
                    menu::Item::Button(
                        fl!("about"),
                        Some(icons::get_handle("info-outline-symbolic", 14)),
                        TweaksAction::About,
                    ),
                ],
            ),
        )])
        .item_height(ItemHeight::Dynamic(40))
        .item_width(ItemWidth::Uniform(240))
        .spacing(4.0);

        vec![menu_bar.into()]
    }
}
