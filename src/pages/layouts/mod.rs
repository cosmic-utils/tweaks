use config::Layout;
use cosmic::{
    iced::{alignment::Horizontal, Length},
    widget, Element, Task,
};
use cosmic_ext_config_templates::load_template;

use crate::{core::grid::GridMetrics, fl, Error};

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
            layouts: Vec::new(),
            selected_layout: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ApplyLayout(Layout),
    LoadLayouts(Vec<Layout>),
}

impl Layouts {
    pub fn view(&self) -> Element<Message> {
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
                        .push(layout.preview(&spacing, item_width))
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

        widget::settings::section()
            .title(fl!("layouts"))
            .add(grid)
            .into()
    }

    pub fn update(&mut self, message: Message) -> Task<crate::app::message::Message> {
        match message {
            Message::LoadLayouts(layouts) => {
                self.layouts = layouts;
            }
            Message::ApplyLayout(layout) => {
                self.selected_layout = Some(layout.clone());
                if let Err(e) = load_template(layout.schema.clone()) {
                    eprintln!("Failed to load template: {}", e);
                }
            }
        }
        Task::none()
    }

    pub fn init() -> Result<(), Error> {
        let layouts_dir = dirs::data_local_dir()
            .map(|path| path.join("cosmic/layouts"))
            .ok_or(Error::LayoutPathNotFound)?;

        if !layouts_dir.exists() {
            std::fs::create_dir_all(&layouts_dir)?;
        }

        let layouts = vec![
            ("cosmic", include_str!("../../../res/layouts/cosmic.ron")),
            ("mac", include_str!("../../../res/layouts/mac.ron")),
            ("windows", include_str!("../../../res/layouts/windows.ron")),
            ("ubuntu", include_str!("../../../res/layouts/ubuntu.ron")),
        ];

        for (name, content) in layouts {
            let file_path = layouts_dir.join(name.to_lowercase()).with_extension("ron");
            std::fs::write(file_path, content)?;
        }

        Ok(())
    }
}
