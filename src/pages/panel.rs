use cosmic::{widget, Element};

#[derive(Debug, Clone)]
pub enum Message {}

pub fn view<'a>() -> Element<'a, Message> {
    widget::container(widget::text::title1("Panel")).into()
}
