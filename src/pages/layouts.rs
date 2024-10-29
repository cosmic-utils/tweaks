use std::path::{Path, PathBuf};

use config::{CustomLayout, Layout, LayoutsConfig};
use cosmic::{cosmic_config::Config, widget, Application, Apply, Element, Task};
use cosmic_ext_config_templates::{generate_template, load_template};
use dirs::data_local_dir;

use crate::app::TweakTool;

pub mod config;

#[derive(Debug)]
pub struct Layouts {
    pub helper: Option<Config>,
    pub config: LayoutsConfig,
}

impl Default for Layouts {
    fn default() -> Self {
        let (helper, config) = (LayoutsConfig::helper(), LayoutsConfig::config());
        Self { helper, config }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectLayout(Layout),
    SaveCurrentLayout(String),
}

impl Layouts {
    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        let spacing = cosmic::theme::active().cosmic().spacing;
        let layouts = self
            .config
            .layouts
            .iter()
            .map(|layout| {
                widget::button::text(layout.name())
                    .on_press(Message::SelectLayout(layout.clone()))
                    .into()
            })
            .collect::<Vec<Element<Message>>>();

        widget::column()
            .push(widget::text("Layouts"))
            .push(
                widget::flex_row(layouts)
                    .row_spacing(spacing.space_xs)
                    .column_spacing(spacing.space_xs)
                    .apply(widget::container)
                    .padding([0, spacing.space_xxs]),
            )
            .into()
    }

    pub fn update(&mut self, message: Message) -> Task<crate::app::Message> {
        match message {
            Message::SelectLayout(layout) => {
                if let Err(e) = load_template(layout.schema().clone()) {
                    eprintln!("Failed to load template: {}", e);
                }
            }
            Message::SaveCurrentLayout(name) => {
                let path = data_local_dir()
                    .unwrap()
                    .join("cosmic")
                    .join(TweakTool::APP_ID)
                    .join("layouts")
                    .join(&name);
                let config_dirs = vec![
                    PathBuf::from("com.system76.CosmicPanel"),
                    PathBuf::from("com.system76.CosmicPanel.Dock"),
                    PathBuf::from("com.system76.CosmicPanel.Panel"),
                    PathBuf::from("com.system76.CosmicPanelButton"),
                ];
                let config_dirs = config_dirs
                    .iter()
                    .map(|p| p.as_path())
                    .collect::<Vec<&Path>>();
                match generate_template(config_dirs, &path) {
                    Ok(_) => {
                        if let Some(helper) = &self.helper {
                            let layout = CustomLayout::new(name, &path);
                            let mut layouts = self.config.layouts.clone();
                            layouts.push(Layout::Custom(layout));
                            match self.config.set_layouts(helper, layouts) {
                                Ok(written) => {
                                    if !written {
                                        eprintln!("Failed to write layouts to config");
                                    }
                                }
                                Err(e) => eprintln!("Failed to set layouts: {}", e),
                            }
                        }
                    }
                    Err(e) => eprintln!("Failed to generate template: {}", e),
                }
            }
        }
        Task::none()
    }
}
