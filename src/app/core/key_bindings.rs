use std::collections::HashMap;

use cosmic::iced::keyboard::Key;
use cosmic::widget::menu::key_bind::KeyBind;
use cosmic::widget::menu::key_bind::Modifier;

use crate::app::action::TweaksAction;

pub struct KeyBindings;

impl KeyBindings {
    pub fn new() -> HashMap<KeyBind, TweaksAction> {
        let mut key_binds = HashMap::new();

        macro_rules! bind {
            ([$($modifier:ident),* $(,)?], $key:expr, $action:ident) => {{
                key_binds.insert(
                    KeyBind {
                        modifiers: vec![$(Modifier::$modifier),*],
                        key: $key,
                    },
                    TweaksAction::$action,
                );
            }};
        }

        bind!([Ctrl], Key::Character(",".into()), Settings);
        bind!([Ctrl], Key::Character("i".into()), About);

        key_binds
    }
}
