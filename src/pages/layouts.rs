use config::{CustomLayout, Layout, LayoutsConfig};
use cosmic::{
    cosmic_config::Config, iced::alignment::Horizontal, widget, Application, Apply, Element, Task,
};
use cosmic_ext_config_templates::{load_template, panel::PanelSchema, Schema};
use dirs::data_local_dir;

use crate::{app::TweakTool, core::icons, fl};

pub mod config;
pub mod preview;

#[derive(Debug)]
pub struct Layouts {
    pub helper: Option<Config>,
    pub config: LayoutsConfig,
    selected_layout: Option<Layout>,
}

impl Default for Layouts {
    fn default() -> Self {
        let (helper, config) = (LayoutsConfig::helper(), LayoutsConfig::config());
        Self {
            helper,
            config,
            selected_layout: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ApplyLayout(Layout),
    SelectLayout(Layout),
    DeleteLayout,
    OpenSaveDialog,
    SaveCurrentLayout(String),
}

impl Layouts {
    pub fn view(&self) -> Element<Message> {
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
            Message::ApplyLayout(layout) => {
                if let Err(e) = load_template(layout.schema().clone()) {
                    eprintln!("Failed to load template: {}", e);
                }
            }
            Message::SelectLayout(layout) => self.selected_layout = Some(layout),
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
                match PanelSchema::generate()
                    .and_then(|panel_schema| Schema::Panel(panel_schema).save(&path))
                {
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
            Message::DeleteLayout => {
                if let Some(layout) = &self.selected_layout {
                    if let Layout::Custom(existing_layout) = &layout {
                        if existing_layout.path().exists() {
                            if let Err(e) = std::fs::remove_file(existing_layout.path()) {
                                log::error!("Failed to delete layout: {}", e);
                                return Task::batch(commands);
                            }
                        }
                        let mut layouts = self.config.layouts.clone();
                        layouts.retain(|l| l != layout);
                        if let Some(helper) = &self.helper {
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
                }
            }
        }
        Task::batch(commands)
    }
}
