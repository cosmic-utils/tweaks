use config::{Snapshot, SnapshotsConfig};
use cosmic::{cosmic_config::Config, widget, Application, Apply, Element, Task};
use cosmic_ext_config_templates::{load_template, panel::PanelSchema, Schema};
use dirs::data_local_dir;

use crate::{app::TweakTool, core::icons, fl};

mod config;

#[derive(Debug)]
pub struct Snapshots {
    pub helper: Option<Config>,
    pub config: SnapshotsConfig,
}

impl Default for Snapshots {
    fn default() -> Self {
        let (helper, config) = (SnapshotsConfig::helper(), SnapshotsConfig::config());
        Self { helper, config }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    CreateSnapshot,
    RestoreSnapshot(Snapshot),
    DeleteSnapshot(Snapshot),
    OpenSaveDialog,
}

impl Snapshots {
    pub fn view(&self) -> Element<Message> {
        let spacing = cosmic::theme::active().cosmic().spacing;
        let mut snapshots = self.config.snapshots.clone();
        snapshots.sort_by(|a, b| {
            b.created()
                .and_utc()
                .timestamp()
                .cmp(&a.created().and_utc().timestamp())
        });
        let snapshots = snapshots
            .iter()
            .map(|snapshot| {
                widget::settings::item(
                    snapshot.name(),
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
                        .spacing(spacing.space_xxs),
                )
                .into()
            })
            .collect::<Vec<Element<Message>>>();

        let snapshots: Element<_> = if snapshots.is_empty() {
            widget::text(fl!("no-snapshots")).into()
        } else {
            widget::settings::section().extend(snapshots).into()
        };
        widget::scrollable(
            widget::column()
                .push(
                    widget::row::with_children(vec![
                        widget::text::title3(fl!("snapshots")).into(),
                        widget::horizontal_space().into(),
                        widget::tooltip::tooltip(
                            icons::get_handle("list-add-symbolic", 16)
                                .apply(widget::button::icon)
                                .padding(spacing.space_xxs)
                                .on_press(Message::OpenSaveDialog)
                                .class(cosmic::style::Button::Standard),
                            widget::text(fl!("create-snapshot")),
                            widget::tooltip::Position::Bottom,
                        )
                        .into(),
                    ])
                    .spacing(spacing.space_xxs),
                )
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
            Message::CreateSnapshot => {
                let path = data_local_dir()
                    .unwrap()
                    .join(TweakTool::APP_ID)
                    .join("snapshots");
                if !path.exists() {
                    if let Err(e) = std::fs::create_dir_all(&path) {
                        log::error!("{e}");
                    }
                }
                let snapshot = Snapshot::new(&path);
                match PanelSchema::generate()
                    .and_then(|panel_schema| Schema::Panel(panel_schema).save(snapshot.path()))
                {
                    Ok(_) => {
                        if let Some(helper) = &self.helper {
                            let mut snapshots = self.config.snapshots.clone();
                            snapshots.push(snapshot);
                            match self.config.set_snapshots(helper, snapshots) {
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
            Message::DeleteSnapshot(snapshot) => {
                if snapshot.path().exists() {
                    if let Err(e) = std::fs::remove_file(snapshot.path()) {
                        log::error!("Failed to delete layout: {}", e);
                        return Task::batch(commands);
                    }
                }
                let mut snapshots = self.config.snapshots.clone();
                snapshots.retain(|l| *l != snapshot);
                if let Some(helper) = &self.helper {
                    match self.config.set_snapshots(helper, snapshots) {
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
        Task::batch(commands)
    }
}
