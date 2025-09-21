use cosmic::{
    Element,
    iced::{Alignment, Length},
    widget,
};

use crate::app::{App, message::Message, page::Page};

use super::Cosmic;

impl Cosmic {
    pub fn view<'a>(app: &'a App) -> Element<'a, Message> {
        let spacing = cosmic::theme::spacing();
        let entity = app.cosmic.nav_model.active();
        let nav_page = app
            .cosmic
            .nav_model
            .data::<Page>(entity)
            .unwrap_or_default();

        let view = match nav_page {
            Page::ColorSchemes => app
                .color_schemes
                .view()
                .map(Box::new)
                .map(Message::ColorSchemes),
            Page::Dock => app.dock.view().map(Message::Dock),
            Page::Panel => app.panel.view().map(Message::Panel),
            Page::Layouts => app.layouts.view().map(Message::Layouts),
            Page::Snapshots => app.snapshots.view().map(Message::Snapshots),
            Page::Shortcuts => app.shortcuts.view().map(Message::Shortcuts),
        };

        widget::column()
            .push(view)
            .padding(spacing.space_xs)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .into()
    }
}
