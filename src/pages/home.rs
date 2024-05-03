use cosmic::{iced::Alignment, widget, Element};

use crate::fl;

#[derive(Debug, Clone)]
pub enum Message {}

pub fn view<'a>() -> Element<'a, Message> {
    let spacing = cosmic::theme::active().cosmic().spacing;
    widget::scrollable(
        widget::column::with_children(vec![
            widget::text::title1(fl!("app-title")).into(),
            widget::text::title4(fl!("app-description")).into(),
        ])
        .align_items(Alignment::Center)
        .spacing(spacing.space_xxs),
    )
    .into()
}
