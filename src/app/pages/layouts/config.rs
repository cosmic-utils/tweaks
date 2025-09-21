use crate::{Error, app::App};

use super::{Message, preview::LayoutPreview};
use cosmic::{Application, Element, widget};
use cosmic_ext_config_templates::{Schema, panel::PanelSchema};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layout {
    pub id: Uuid,
    pub name: String,
    pub custom: bool,
    pub schema: Schema,
    pub preview: LayoutPreview,
}

impl Layout {
    pub fn new(name: String, preview: LayoutPreview) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            custom: true,
            schema: Schema::Panel(PanelSchema::generate().unwrap()),
            preview,
        }
    }

    pub fn preview<'a>(
        &'a self,
        spacing: &cosmic::cosmic_theme::Spacing,
        item_width: usize,
        preview_height: u16,
        selected_layout: &Option<Layout>,
    ) -> Element<'a, Message> {
        let mut button = widget::button::custom(self.preview.view(spacing, preview_height))
            .on_press(Message::Select(self.clone()))
            .class(cosmic::style::Button::Image)
            .width(item_width as f32);
        if let Some(selected) = selected_layout {
            button = button.selected(selected.name == self.name);
        }
        button.into()
    }

    pub fn list() -> Result<Vec<Layout>, Error> {
        let mut layouts = Vec::new();
        let layouts_dir = dirs::data_local_dir()
            .map(|path| path.join(App::APP_ID).join("layouts"))
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
