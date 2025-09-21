use cosmic::{
    ApplicationExt, Task,
    app::{self},
    widget::{self},
};

use crate::app::App;
use crate::app::message::Message;
use crate::app::page::Page;
use crate::fl;

use super::Cosmic;

impl Cosmic {
    pub fn on_nav_select(app: &mut App, id: widget::nav_bar::Id) -> app::Task<Message> {
        app.cosmic.nav_model.activate(id);

        let title = if let Some(page) = app.cosmic.nav_model.data::<Page>(id) {
            format!("{} - {}", page.title(), fl!("app-title"))
        } else {
            fl!("app-title")
        };

        Task::batch(vec![app.set_window_title(title)])
    }
}
