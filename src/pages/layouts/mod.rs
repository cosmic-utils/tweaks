use config::Layout;
use cosmic::{iced::alignment::Horizontal, widget, Apply, Element, Task};
use cosmic_ext_config_templates::load_template;

use crate::fl;

pub mod config;
pub mod preview;

#[derive(Debug)]
pub struct Layouts {
    layouts: Vec<Layout>,
    selected_layout: Option<Layout>,
}

impl Default for Layouts {
    fn default() -> Self {
        Self {
            layouts: Layout::list(),
            selected_layout: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ApplyLayout(Layout),
}

impl Layouts {
    pub fn view(&self) -> Element<Message> {
        let spacing = cosmic::theme::spacing();
        let layouts = self
            .layouts
            .iter()
            .map(|layout| {
                widget::column()
                    .push(layout.preview())
                    .push(widget::text(layout.name()))
                    .spacing(spacing.space_xs)
                    .align_x(Horizontal::Center)
                    .into()
            })
            .collect::<Vec<Element<Message>>>();

        widget::settings::section()
            .title(fl!("layouts"))
            .add(widget::scrollable(
                widget::flex_row(layouts)
                    .row_spacing(spacing.space_s)
                    .column_spacing(spacing.space_s)
                    .apply(widget::container)
                    .padding([0, spacing.space_xxs]),
            ))
            .into()
    }

    pub fn update(&mut self, message: Message) -> Task<crate::app::message::Message> {
        match message {
            Message::ApplyLayout(layout) => {
                self.selected_layout = Some(layout.clone());
                if let Err(e) = load_template(layout.schema().clone()) {
                    eprintln!("Failed to load template: {}", e);
                }
            }
        }
        Task::none()
    }
}
