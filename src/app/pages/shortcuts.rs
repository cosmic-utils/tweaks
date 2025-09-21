use std::{collections::HashMap, io};

use cosmic::{
    Element, Task,
    cosmic_config::{self, ConfigGet, ConfigSet},
    iced::{alignment::Vertical, padding},
    widget::{button, column, horizontal_space, row, text, vertical_space},
};
use cosmic_settings_config::{Shortcuts, shortcuts};

pub struct ShortcutsPage {
    pub config: cosmic_config::Config,
}

#[derive(Debug, Clone)]
enum ShortcutsGroup {
    Windows,
}

impl ShortcutsGroup {
    fn name(&self) -> &'static str {
        match self {
            ShortcutsGroup::Windows => "Windows",
        }
    }

    fn desc(&self) -> String {
        match self {
            ShortcutsGroup::Windows => fl!("windows-desc"),
        }
    }

    fn shortcuts(&self) -> HashMap<shortcuts::Binding, shortcuts::Action> {
        let str = match self {
            Self::Windows => {
                include_str!("../../../res/shortcuts/windows.ron")
            }
        };

        ron::de::from_str(str).unwrap()
    }
}

#[derive(Debug, Clone)]
#[allow(private_interfaces)]
pub enum Message {
    ApplyShortcuts(ShortcutsGroup),
}

impl ShortcutsPage {
    pub fn new() -> Self {
        Self {
            config: shortcuts::context().unwrap(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<crate::app::message::Message> {
        match message {
            Message::ApplyShortcuts(shortcuts_group) => {
                let mut shortcuts = match self.config.get::<Shortcuts>("custom") {
                    Ok(shortcuts) => shortcuts,
                    Err(cosmic_config::Error::GetKey(_, e))
                        if e.kind() == io::ErrorKind::NotFound =>
                    {
                        Shortcuts::default()
                    }
                    Err(e) => {
                        error!("unable to get the current shortcuts config: {e}");
                        Shortcuts::default()
                    }
                };

                shortcuts.0.extend(shortcuts_group.shortcuts());

                if let Err(e) = self.config.set("custom", shortcuts) {
                    error!("failed to write shortcuts config: {e}");
                }
            }
        }
        Task::none()
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        column()
            .push(text::heading(fl!("warning")))
            .push(vertical_space().height(25))
            .push(
                column().spacing(5).push(
                    row()
                        .push(view_button(ShortcutsGroup::Windows))
                        .push(view_button(ShortcutsGroup::Windows)),
                ),
            )
            .into()
    }
}

fn view_button<'a>(shortcuts: ShortcutsGroup) -> Element<'a, Message> {
    button::custom(
        row()
            .align_y(Vertical::Center)
            .padding(5)
            .push(text(shortcuts.name()))
            .push(horizontal_space())
            .push(text(shortcuts.desc())),
    )
    .padding(padding::all(10))
    .on_press(Message::ApplyShortcuts(shortcuts))
    .into()
}
