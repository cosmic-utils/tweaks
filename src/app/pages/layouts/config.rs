use crate::Error;

use super::{preview::LayoutPreview, Message};
use cosmic::{widget, Element};
use cosmic_ext_config_templates::{panel::PanelSchema, Schema};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layout {
    pub name: String,
    pub schema: Schema,
    pub preview: LayoutPreview,
}

impl Layout {
    pub fn new(name: String, preview: LayoutPreview) -> Self {
        Self {
            name,
            schema: Schema::Panel(PanelSchema::generate().unwrap()),
            preview,
        }
    }

    pub fn preview(
        &self,
        spacing: &cosmic::cosmic_theme::Spacing,
        item_width: usize,
        preview_height: u16,
    ) -> Element<Message> {
        widget::button::custom(self.preview.view(&spacing, preview_height))
            .on_press(Message::ApplyLayout(self.clone()))
            .class(cosmic::style::Button::Image)
            .width(item_width as f32)
            .into()
    }

    pub fn list() -> Result<Vec<Layout>, Error> {
        let mut layouts = Vec::new();
        let layouts_dir = dirs::data_local_dir()
            .map(|path| path.join("cosmic/layouts"))
            .ok_or(Error::LayoutPathNotFound)?;

        if let Ok(entries) = std::fs::read_dir(layouts_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("ron") {
                    let contents = std::fs::read_to_string(&path)?;
                    let layout = ron::from_str::<Layout>(&contents)?;
                    layouts.push(layout);
                }
            }
        }
        Ok(layouts)
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            name: String::new(),
            schema: Schema::Panel(PanelSchema::generate().unwrap()),
            preview: LayoutPreview::default(),
        }
    }
}
