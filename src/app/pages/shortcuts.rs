use std::{env, fs, path::Path};

use cosmic::{
    iced::padding,
    widget::{button, column, horizontal_space, row, text, vertical_space},
    Element, Task,
};

use crate::fl;

pub struct Shortcuts {}

#[derive(Debug, Clone)]
enum Shortcut {
    Windows,
}

impl Shortcut {
    fn name(&self) -> &'static str {
        match self {
            Shortcut::Windows => "Windows",
        }
    }

    fn desc(&self) -> String {
        match self {
            Shortcut::Windows => fl!("windows-desc"),
        }
    }

    fn schema(&self) -> &'static str {
        match self {
            Self::Windows => include_str!("../../../res/shortcuts/windows.ron"),
        }
    }
}

#[derive(Debug, Clone)]
#[allow(private_interfaces)]
pub enum Message {
    ApplyShortcuts(Shortcut),
}

impl Shortcuts {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: Message) -> Task<crate::app::message::Message> {
        match message {
            Message::ApplyShortcuts(shortcut) => {
                let path = Path::new(&env::var("HOME").unwrap())
                    .join(".config/cosmic/com.system76.CosmicSettings.Shortcuts/v1/custom");

                if let Err(e) = fs::write(&path, shortcut.schema()) {
                    eprintln!("Failed to write shortcuts: {}", e);
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
                        .push(view_button(Shortcut::Windows))
                        .push(view_button(Shortcut::Windows)),
                ),
            )
            .into()
    }
}

fn view_button<'a>(shortcuts: Shortcut) -> Element<'a, Message> {
    button::custom(
        row()
            .push(text(shortcuts.name()))
            .push(horizontal_space())
            .push(text(shortcuts.desc())),
    )
    .padding(padding::all(10))
    .on_press(Message::ApplyShortcuts(shortcuts))
    .into()
}
