use config::Layout;
use cosmic::{
    Application, Element, Task,
    iced::{Length, alignment::Horizontal},
    widget::{
        self,
        segmented_button::{self, SingleSelect},
    },
};
use cosmic_ext_config_templates::load_template;
use preview::{LayoutPreview, Position};

use crate::app::{App, core::grid::GridMetrics};
use crate::{Error, fl};

pub mod config;
pub mod dialog;
pub mod preview;

pub struct Layouts {
    layouts: Vec<Layout>,
    pub selected_layout: Option<Layout>,
    pub panel_model: segmented_button::Model<SingleSelect>,
    pub dock_model: segmented_button::Model<SingleSelect>,
}

impl Default for Layouts {
    fn default() -> Self {
        Self {
            layouts: Vec::new(),
            selected_layout: None,
            panel_model: segmented_button::Model::builder()
                .insert(|b| b.text(fl!("left")).data(Position::Left))
                .insert(|b| b.text(fl!("top")).data(Position::Top).activate())
                .insert(|b| b.text(fl!("right")).data(Position::Right))
                .insert(|b| b.text(fl!("bottom")).data(Position::Bottom))
                .build(),
            dock_model: segmented_button::Model::builder()
                .insert(|b| b.text(fl!("left")).data(Position::Left))
                .insert(|b| b.text(fl!("top")).data(Position::Top))
                .insert(|b| b.text(fl!("right")).data(Position::Right))
                .insert(|b| b.text(fl!("bottom")).data(Position::Bottom).activate())
                .build(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Select(Layout),
    Apply,
    Delete,
    LoadLayouts(Vec<Layout>),
    Create(String, LayoutPreview),
}

impl Layouts {
    pub fn init() -> Result<(), Error> {
        let layouts_dir = dirs::data_local_dir()
            .map(|path| path.join(App::APP_ID).join("layouts"))
            .ok_or(Error::LayoutPathNotFound)?;

        if !layouts_dir.exists() {
            std::fs::create_dir_all(&layouts_dir)?;
        }

        let layouts = vec![
            ("cosmic", include_str!("../../../../res/layouts/cosmic.ron")),
            ("mac", include_str!("../../../../res/layouts/mac.ron")),
            (
                "windows",
                include_str!("../../../../res/layouts/windows.ron"),
            ),
            ("ubuntu", include_str!("../../../../res/layouts/ubuntu.ron")),
        ];

        for (name, content) in layouts {
            let file_path = layouts_dir.join(name.to_lowercase()).with_extension("ron");
            std::fs::write(file_path, content)?;
        }

        Ok(())
    }

    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        let spacing = cosmic::theme::spacing();
        let grid = widget::responsive(move |size| {
            let GridMetrics {
                cols,
                item_width,
                column_spacing,
            } = GridMetrics::custom(&spacing, size.width as usize);

            let mut grid = widget::grid();
            let mut col = 0;
            for layout in self.layouts.iter() {
                if col >= cols {
                    grid = grid.insert_row();
                    col = 0;
                }
                grid = grid.push(
                    widget::column()
                        .push(layout.preview(&spacing, item_width, 130, &self.selected_layout))
                        .push(widget::text(&layout.name))
                        .spacing(spacing.space_xs)
                        .align_x(Horizontal::Center),
                );
                col += 1;
            }
            widget::scrollable(
                grid.column_spacing(column_spacing)
                    .row_spacing(column_spacing),
            )
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
        });

        widget::column()
            .push(widget::settings::section().title(fl!("layouts")).add(grid))
            .into()
    }

    pub fn update(&mut self, message: Message) -> Task<crate::app::message::Message> {
        match message {
            Message::LoadLayouts(layouts) => {
                self.layouts = layouts;
            }
            Message::Select(layout) => {
                self.selected_layout = Some(layout.clone());
            }
            Message::Apply => {
                if let Some(layout) = &self.selected_layout {
                    if let Err(e) = load_template(layout.schema.clone()) {
                        eprintln!("Failed to load template: {}", e);
                    }
                    self.selected_layout = None;
                }
            }
            Message::Delete => {
                if let Some(layout) = self.selected_layout.clone() {
                    let layouts_dir = dirs::data_local_dir()
                        .unwrap()
                        .join(App::APP_ID)
                        .join("layouts");
                    let file_path = layouts_dir
                        .join(layout.id.to_string())
                        .with_extension("ron");
                    if file_path.exists() {
                        match std::fs::remove_file(file_path) {
                            Ok(_) => {
                                self.selected_layout = None;
                                self.layouts = self
                                    .layouts
                                    .clone()
                                    .into_iter()
                                    .filter(|l| l.id != layout.id)
                                    .collect();
                            }
                            Err(e) => {
                                eprintln!("Failed to delete layout: {}", e);
                            }
                        }
                    }
                }
            }
            Message::Create(name, preview) => {
                let layout = Layout::new(name, preview);

                let layouts_dir = dirs::data_local_dir()
                    .unwrap()
                    .join(App::APP_ID)
                    .join("layouts");

                let file_path = layouts_dir
                    .join(layout.id.to_string())
                    .with_extension("ron");
                if file_path.exists() {
                    return Task::none();
                }

                match std::fs::write(&file_path, ron::to_string(&layout).unwrap()) {
                    Ok(_) => match crate::app::pages::layouts::config::Layout::list() {
                        Ok(layouts) => self.layouts = layouts,
                        Err(e) => eprintln!("Failed to reload layouts: {e}"),
                    },
                    Err(e) => {
                        log::error!("Failed to write layout: {}", e);
                    }
                };
            }
        }
        Task::none()
    }
}
