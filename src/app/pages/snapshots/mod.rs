use config::Snapshot;
use cosmic::{Application, Element, Task, iced::Length, widget};
use cosmic_ext_config_templates::load_template;
use dirs::data_local_dir;

use crate::app::core::icons;
use crate::app::pages;
use crate::app::pages::snapshots::config::SnapshotKind;
use crate::{app::App, fl};

pub mod config;

#[derive(Debug, Default)]
pub struct Snapshots {
    snapshots: Vec<Snapshot>,
}

impl Snapshots {
    pub fn list() -> Vec<Snapshot> {
        dirs::data_local_dir()
            .expect("Failed to get data directory")
            .join(App::APP_ID)
            .join("snapshots")
            .read_dir()
            .expect("Failed to read snapshots directory")
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| std::fs::read_to_string(entry.path()).ok())
            .filter_map(|entry| ron::from_str(&entry).ok())
            .collect()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    CreateSnapshot(String, SnapshotKind),
    ReloadSnapshots,
    RestoreSnapshot(Snapshot),
    DeleteSnapshot(Snapshot),
}

impl Snapshots {
    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        let spacing = cosmic::theme::spacing();
        let snapshots = self
            .snapshots
            .iter()
            .map(|snapshot| {
                widget::settings::item_row(vec![
                    widget::text(&snapshot.name)
                        .width(Length::FillPortion(2))
                        .into(),
                    widget::text(snapshot.kind.to_string())
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
                        .spacing(spacing.space_xxs),
                )
                .push(widget::text::body("Each time you open Tweaks, we save the current state of your desktop, if you ever break it, simply restore it."))
                .push_maybe(header)
                .push(snapshots)
                .spacing(spacing.space_xs),
        )
        .into()
    }

    pub fn update(&mut self, message: Message) -> Task<crate::app::message::Message> {
        let mut tasks = vec![];
        match message {
            Message::ReloadSnapshots => {
                self.snapshots = Snapshots::list();
                self.snapshots.sort_by(|a, b| {
                    b.created
                        .and_utc()
                        .timestamp()
                        .cmp(&a.created.and_utc().timestamp())
                });
            }
            Message::RestoreSnapshot(snapshot) => {
                if let Some(schema) = snapshot.schema {
                    if let Err(e) = load_template(schema) {
                        eprintln!("Failed to load template: {}", e);
                    }
                } else {
                    log::warn!("Snapshot does not contain a valid schema.");
                }

                tasks.push(cosmic::task::message(crate::app::Message::ColorSchemes(
                    Box::new(pages::color_schemes::Message::SetColorScheme(
                        snapshot.color_scheme,
                    )),
                )));
            }
            Message::CreateSnapshot(name, kind) => {
                let path = data_local_dir()
                    .unwrap()
                    .join(App::APP_ID)
                    .join("snapshots");
                if !path.exists()
                    && let Err(e) = std::fs::create_dir_all(&path)
                {
                    log::error!("{e}");
                }
                let snapshot = Snapshot::new(name, kind);
                match ron::to_string(&snapshot) {
                    Ok(data) => {
                        if let Err(e) = std::fs::write(snapshot.path(), data) {
                            log::error!("Failed to write snapshot: {}", e);
                        }
                        log::info!("Snapshot created: {}", snapshot.name);
                        tasks.push(self.update(Message::ReloadSnapshots));
                    }
                    Err(e) => {
                        log::error!("Failed to serialize snapshot: {}", e);
                    }
                }
            }
            Message::DeleteSnapshot(snapshot) => {
                if snapshot.path().exists() {
                    if let Err(e) = std::fs::remove_file(snapshot.path()) {
                        log::error!("Failed to delete layout: {}", e);
                        return Task::batch(tasks);
                    }
                    tasks.push(self.update(Message::ReloadSnapshots));
                }
            }
        }
        Task::batch(tasks)
    }
}
