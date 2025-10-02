use cosmic::Element;
use cosmic::widget::RcElementWrapper;
use cosmic::widget::menu::{self, ItemHeight, ItemWidth};

use crate::app::App;
use crate::app::action::TweaksAction;
use crate::app::message::Message;

use super::Cosmic;
use crate::{fl, icon_handle};

impl Cosmic {
    pub fn header_start<'a>(app: &'a App) -> Vec<Element<'a, Message>> {
        let menu_bar = menu::bar(vec![menu::Tree::with_children(
            RcElementWrapper::new(menu::root(fl!("view")).into()),
            menu::items(
                &app.cosmic.key_binds,
                vec![
                    menu::Item::Button(
                        fl!("settings"),
                        Some(icon_handle!("settings-symbolic", 14)),
                        TweaksAction::Settings,
                    ),
                    menu::Item::Divider,
                    menu::Item::Button(
                        fl!("about"),
                        Some(icon_handle!("info-outline-symbolic", 14)),
                        TweaksAction::About,
                    ),
                ],
            ),
        )])
        .item_height(ItemHeight::Dynamic(40))
        .item_width(ItemWidth::Uniform(240))
        .spacing(4.0);

        vec![Element::from(menu_bar)]
    }
}
