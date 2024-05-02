use cosmic::{widget, Element};

use crate::fl;

#[derive(Debug, Clone)]
pub enum Message {}

pub fn view<'a>() -> Element<'a, Message> {
    widget::container(widget::text::title1(fl!("app-title"))).into()
}
