use cosmic::{Element, iced::Alignment, widget};

use crate::app::dialog::DialogPage;
use crate::app::message::Message;

use crate::app::core::icons;
use crate::app::pages::layouts::preview::{LayoutPreview, PanelProperties};
use crate::fl;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateLayoutDialog {
    pub name: String,
    pub preview: LayoutPreview,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelType {
    Panel,
    Dock,
}

impl CreateLayoutDialog {
    pub fn new(name: String, preview: LayoutPreview, error: Option<String>) -> Self {
        Self {
            name,
            preview,
            error,
        }
    }

    pub fn section<'a>(
        &'a self,
        panel_type: PanelType,
        model: &'a widget::segmented_button::SingleSelectModel,
    ) -> widget::settings::Section<'a, Message> {
        let title = match panel_type {
            PanelType::Panel => fl!("panel"),
            PanelType::Dock => fl!("dock"),
        };

        let current_props = match panel_type {
            PanelType::Panel => &self.preview.panel,
            PanelType::Dock => &self.preview.dock,
        };

        let mut section = widget::settings::section()
            .title(title)
            .add(self.create_show_toggle(panel_type, current_props))
            .add(self.create_extend_toggle(panel_type, current_props))
            .add(self.create_position_control(panel_type, model))
            .add(self.create_size_control(panel_type, current_props));

        if panel_type == PanelType::Dock {
            section = section.add(self.create_dock_icons_control());
        }

        section
    }

    pub fn create_show_toggle<'a>(
        &'a self,
        panel_type: PanelType,
        current_props: &'a PanelProperties,
    ) -> impl Into<Element<'a, Message>> {
        widget::settings::item::builder(fl!("show"))
            .icon(icons::get_icon("resize-mode-symbolic", 18))
            .control(
                widget::toggler(!current_props.hidden).on_toggle(move |hidden| {
                    let mut new_preview = self.preview;
                    let new_props = PanelProperties {
                        hidden: !hidden,
                        ..*current_props
                    };

                    match panel_type {
                        PanelType::Panel => {
                            new_preview.panel = new_props;
                        }
                        PanelType::Dock => {
                            new_preview.dock = new_props;
                        }
                    }

                    Message::DialogUpdate(DialogPage::CreateLayout(CreateLayoutDialog::new(
                        self.name.to_string(),
                        new_preview,
                        self.error.clone(),
                    )))
                }),
            )
    }

    pub fn create_extend_toggle<'a>(
        &'a self,
        panel_type: PanelType,
        current_props: &'a PanelProperties,
    ) -> impl Into<Element<'a, Message>> {
        widget::settings::item::builder(fl!("extend"))
            .icon(icons::get_icon("resize-mode-symbolic", 18))
            .control(
                widget::toggler(current_props.extend).on_toggle(move |extend| {
                    let mut new_preview = self.preview;
                    let new_props = PanelProperties {
                        extend,
                        ..*current_props
                    };

                    match panel_type {
                        PanelType::Panel => {
                            new_preview.panel = new_props;
                        }
                        PanelType::Dock => {
                            new_preview.dock = new_props;
                        }
                    }

                    Message::DialogUpdate(DialogPage::CreateLayout(CreateLayoutDialog::new(
                        self.name.to_string(),
                        new_preview,
                        self.error.clone(),
                    )))
                }),
            )
    }

    pub fn create_position_control<'a>(
        &'a self,
        panel_type: PanelType,
        model: &'a widget::segmented_button::SingleSelectModel,
    ) -> impl Into<Element<'a, Message>> {
        let spacing = cosmic::theme::spacing();
        let name = self.name.to_string().clone();
        let preview = self.preview;
        let panel_type = panel_type;

        widget::settings::item::builder(fl!("position"))
            .icon(icons::get_icon("resize-mode-symbolic", 18))
            .control(
                widget::segmented_button::horizontal(model)
                    .on_activate(move |entity| match panel_type {
                        PanelType::Panel => {
                            Message::UpdatePanelLayoutPosition(entity, name.clone(), preview)
                        }
                        PanelType::Dock => {
                            Message::UpdateDockLayoutPosition(entity, name.clone(), preview)
                        }
                    })
                    .button_alignment(Alignment::Center)
                    .button_spacing(spacing.space_xxs),
            )
    }

    pub fn create_size_control<'a>(
        &'a self,
        panel_type: PanelType,
        panel_props: &PanelProperties,
    ) -> impl Into<Element<'a, Message>> {
        let name = self.name.to_string();
        let preview = self.preview;
        let error = self.error.clone();
        let panel_props = *panel_props;
        let panel_type = panel_type;

        widget::settings::item::builder(fl!("size"))
            .icon(icons::get_icon("resize-mode-symbolic", 18))
            .control(widget::spin_button(
                panel_props.size.to_string(),
                panel_props.size as f32,
                1.0,
                0.0,
                50.0,
                move |size| {
                    let mut new_preview = preview;
                    let new_props = PanelProperties {
                        size: size as usize,
                        ..panel_props
                    };

                    match panel_type {
                        PanelType::Panel => {
                            new_preview.panel = new_props;
                        }
                        PanelType::Dock => {
                            new_preview.dock = new_props;
                        }
                    }

                    Message::DialogUpdate(DialogPage::CreateLayout(CreateLayoutDialog::new(
                        name.clone(),
                        new_preview,
                        error.clone(),
                    )))
                },
            ))
    }

    pub fn create_dock_icons_control<'a>(&'a self) -> impl Into<Element<'a, Message>> {
        let name = self.name.to_string();
        let preview = self.preview;
        let error = self.error.clone();

        widget::settings::item::builder(fl!("dock-icons"))
            .icon(icons::get_icon("resize-mode-symbolic", 18))
            .control(widget::spin_button(
                preview.dock_icons.to_string(),
                preview.dock_icons,
                1,
                1,
                20,
                move |size| {
                    Message::DialogUpdate(DialogPage::CreateLayout(CreateLayoutDialog::new(
                        name.clone(),
                        LayoutPreview {
                            dock_icons: size,
                            ..preview
                        },
                        error.clone(),
                    )))
                },
            ))
    }
}
