use std::path::{Path, PathBuf};

use config::{CustomLayout, Layout, LayoutsConfig};
use cosmic::{
    cosmic_config::Config, iced::alignment::Horizontal, widget, Application, Apply, Element, Task,
};
use cosmic_ext_config_templates::{generate_template, load_template};
use dirs::data_local_dir;

use crate::{app::TweakTool, core::icons, fl};

pub mod config;
pub mod factory;

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
    OpenSaveDialog,
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
                widget::column()
                    .push(layout.preview())
                    .push(widget::text(layout.name()))
                    .spacing(spacing.space_xs)
                    .align_x(Horizontal::Center)
                    .into()
            })
            .collect::<Vec<Element<Message>>>();

        widget::scrollable(widget::column::with_children(vec![
            widget::row::with_children(vec![
                widget::text::title3(fl!("layouts")).into(),
                widget::horizontal_space().into(),
                widget::tooltip::tooltip(
                    icons::get_handle("arrow-into-box-symbolic", 16)
                        .apply(widget::button::icon)
                        .padding(spacing.space_xxs)
                        .on_press(Message::OpenSaveDialog)
                        .class(cosmic::style::Button::Standard),
                    widget::text(fl!("save-current-layout")),
                    widget::tooltip::Position::Bottom,
                )
                .into(),
            ])
            .spacing(spacing.space_xxs)
            .into(),
            widget::settings::section()
                .title("Default")
                .add(
                    widget::flex_row(layouts)
                        .row_spacing(spacing.space_s)
                        .column_spacing(spacing.space_s)
                        .apply(widget::container)
                        .padding([0, spacing.space_xxs]),
                )
                .into(),
        ]))
        .into()
    }

    pub fn update(&mut self, message: Message) -> Task<crate::app::Message> {
        let mut commands = vec![];
        match message {
            Message::SelectLayout(layout) => {
                if let Err(e) = load_template(layout.schema().clone()) {
                    eprintln!("Failed to load template: {}", e);
                }
            }
            Message::OpenSaveDialog => commands.push(self.update(Message::OpenSaveDialog)),
            Message::SaveCurrentLayout(name) => {
                let path = data_local_dir()
                    .unwrap()
                    .join(TweakTool::APP_ID)
                    .join("layouts");

                if !path.exists() {
                    if let Err(e) = std::fs::create_dir_all(&path) {
                        log::error!("{e}");
                    }
                }
                let mut path = path.join(&name);
                path.set_extension("ron");
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
                                        log::error!("Failed to write layouts to config");
                                    }
                                }
                                Err(e) => log::error!("Failed to set layouts: {}", e),
                            }
                        }
                    }
                    Err(e) => log::error!("Failed to generate template: {}", e),
                }
            }
        }
        Task::batch(commands)
    }
}
