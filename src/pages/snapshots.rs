use config::{Snapshot, SnapshotKind, SnapshotsConfig};
use cosmic::{iced::Length, widget, Application, Apply, Element, Task};
use cosmic_ext_config_templates::{load_template, panel::PanelSchema, Schema};
use dirs::data_local_dir;

use crate::{app::App, core::icons, fl};

pub mod config;

#[derive(Debug)]
pub struct Snapshots {
    pub config: SnapshotsConfig,
}

impl Default for Snapshots {
    fn default() -> Self {
        Self {
            config: SnapshotsConfig::config(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    CreateSnapshot(String, SnapshotKind),
    RestoreSnapshot(Snapshot),
    DeleteSnapshot(Snapshot),
    OpenSaveDialog,
}

impl Snapshots {
    pub fn view(&self) -> Element<Message> {
        let spacing = cosmic::theme::active().cosmic().spacing;

        let snapshots = self
            .config
            .snapshots
            .iter()
            .map(|snapshot| {
                widget::settings::item_row(vec![
                    widget::text(&snapshot.name)
                        .width(Length::FillPortion(2))
                        .into(),
                    widget::text(snapshot.kind())
                        .width(Length::FillPortion(1))
                        .into(),
                    widget::text(snapshot.created())
                        .width(Length::FillPortion(1))
                        .into(),
                    widget::row()
                        .push(widget::tooltip(
                            widget::button::icon(icons::get_handle(
                                "arrow-circular-bottom-right-symbolic",
                                14,
                            ))
                            .class(cosmic::style::Button::Standard)
                            .on_press(Message::RestoreSnapshot(snapshot.clone())),
                            widget::text(fl!("restore-snapshot")),
                            widget::tooltip::Position::Bottom,
                        ))
                        .push(widget::tooltip(
                            widget::button::icon(icons::get_handle("user-trash-symbolic", 14))
                                .class(cosmic::style::Button::Destructive)
                                .on_press(Message::DeleteSnapshot(snapshot.clone())),
                            widget::text(fl!("delete-snapshot")),
                            widget::tooltip::Position::Bottom,
                        ))
                        .align_y(cosmic::iced::Alignment::Center)
                        .spacing(spacing.space_xxs)
                        .width(Length::FillPortion(1))
                        .into(),
                ])
                .align_y(cosmic::iced::Alignment::Center)
                .spacing(spacing.space_xxxs)
                .width(Length::Fill)
                .into()
            })
            .collect::<Vec<Element<Message>>>();

        let heading_item = |name, width| {
            widget::row()
                .push(widget::text::heading(name))
                .align_y(cosmic::iced::Alignment::Center)
                .spacing(spacing.space_xxxs)
                .width(width)
        };

        let header = if snapshots.is_empty() {
            None
        } else {
            Some(
                widget::row()
                    .push(heading_item(fl!("name"), Length::FillPortion(2)))
                    .push(heading_item(fl!("type"), Length::FillPortion(1)))
                    .push(heading_item(fl!("created"), Length::FillPortion(1)))
                    .push(heading_item(fl!("actions"), Length::FillPortion(1)))
                    .padding([0, spacing.space_m]),
            )
        };

        let snapshots: Element<_> = if snapshots.is_empty() {
            widget::text(fl!("no-snapshots")).into()
        } else {
            widget::settings::section().extend(snapshots).into()
        };

        widget::scrollable(
            widget::column()
                .push(
                    widget::row()
                        .push(widget::text::title3(fl!("snapshots")))
                        .push(widget::horizontal_space())
                        .push(widget::tooltip::tooltip(
                            icons::get_handle("list-add-symbolic", 16)
                                .apply(widget::button::icon)
                                .padding(spacing.space_xxs)
                                .on_press(Message::OpenSaveDialog)
                                .class(cosmic::style::Button::Standard),
                            widget::text(fl!("create-snapshot")),
                            widget::tooltip::Position::Bottom,
                        ))
                        .spacing(spacing.space_xxs),
                )
                .push_maybe(header)
                .push(snapshots)
                .spacing(spacing.space_xs),
        )
        .into()
    }

    pub fn update(&mut self, message: Message) -> Task<crate::app::Message> {
        let mut commands = vec![];
        match message {
            Message::RestoreSnapshot(snapshot) => {
                if let Err(e) = load_template(snapshot.schema().clone()) {
                    eprintln!("Failed to load template: {}", e);
                }
            }
            Message::OpenSaveDialog => commands.push(self.update(Message::OpenSaveDialog)),
            Message::CreateSnapshot(name, kind) => {
                let path = data_local_dir()
                    .unwrap()
                    .join(App::APP_ID)
                    .join("snapshots");
                if !path.exists() {
                    if let Err(e) = std::fs::create_dir_all(&path) {
                        log::error!("{e}");
                    }
                }
                let snapshot = Snapshot::new(&name, &path, kind);
                match PanelSchema::generate()
                    .and_then(|panel_schema| Schema::Panel(panel_schema).save(&snapshot.path))
                {
                    Ok(_) => {
                        let mut snapshots = self.config.snapshots.clone();
                        snapshots.push(snapshot.clone());
                        snapshots.sort_by(|a, b| {
                            b.created
                                .and_utc()
                                .timestamp()
                                .cmp(&a.created.and_utc().timestamp())
                        });
                        match self
                            .config
                            .set_snapshots(&SnapshotsConfig::helper(), snapshots)
                        {
                            Ok(written) => {
                                if !written {
                                    log::error!("Failed to write snapshots to config");
                                }
                            }
                            Err(e) => log::error!("Failed to set snapshots: {}", e),
                        }
                    }
                    Err(e) => log::error!("Failed to generate template: {}", e),
                }
            }
            Message::DeleteSnapshot(snapshot) => {
                if snapshot.path.exists() {
                    if let Err(e) = std::fs::remove_file(&snapshot.path) {
                        log::error!("Failed to delete layout: {}", e);
                        return Task::batch(commands);
                    }
                }
                let mut snapshots = self.config.snapshots.clone();
                snapshots.retain(|l| *l != snapshot);
                snapshots.sort_by(|a, b| {
                    b.created
                        .and_utc()
                        .timestamp()
                        .cmp(&a.created.and_utc().timestamp())
                });
                match self
                    .config
                    .set_snapshots(&SnapshotsConfig::helper(), snapshots)
                {
                    Ok(written) => {
                        if !written {
                            log::error!("Failed to write snapshots to config");
                        }
                    }
                    Err(e) => log::error!("Failed to set snapshots: {}", e),
                }
            }
        }
        Task::batch(commands)
    }
}
