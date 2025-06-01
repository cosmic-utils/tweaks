use crate::Error;

use super::{preview::LayoutPreview, Message};
use cosmic::{widget, Element};
use cosmic_ext_config_templates::Schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layout {
    pub name: String,
    pub schema: Schema,
    pub layout: LayoutPreview,
}

impl Layout {
    pub fn preview(
        &self,
        spacing: &cosmic::cosmic_theme::Spacing,
        item_width: usize,
    ) -> Element<Message> {
        widget::button::custom(self.layout.view(&spacing))
            .on_press(Message::ApplyLayout(self.clone()))
            .class(cosmic::style::Button::Image)
            .width(item_width as f32)
            .into()
    }

    pub(crate) fn list() -> Result<Vec<Layout>, Error> {
        let mut layouts = Vec::new();
        let layouts_dir = dirs::data_local_dir()
            .map(|path| path.join("cosmic/layouts"))
            .ok_or(Error::LayoutPathNotFound)?;

        if !layouts_dir.exists() {
            std::fs::create_dir_all(&layouts_dir)?;
        }

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
