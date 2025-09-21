use super::{Message, context::ContextPage};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TweaksAction {
    About,
    Settings,
}

impl cosmic::widget::menu::Action for TweaksAction {
    type Message = Message;
    fn message(&self) -> Self::Message {
        match self {
            TweaksAction::About => Message::ToggleContextPage(ContextPage::About),
            TweaksAction::Settings => Message::ToggleContextPage(ContextPage::Settings),
        }
    }
}
