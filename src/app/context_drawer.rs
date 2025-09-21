use cosmic::{
    Application,
    app::{self, context_drawer::ContextDrawer},
};

use crate::app::App;
use crate::app::context::ContextPage;
use crate::app::message::Message;

use super::Cosmic;

impl Cosmic {
    pub fn context_drawer<'a>(app: &'a App) -> Option<ContextDrawer<'a, Message>> {
        if !app.core().window.show_context {
            return None;
        }

        Some(match app.cosmic.context_page {
            ContextPage::About => app::context_drawer::about(
                &app.cosmic.about,
                Message::Open,
                Message::ToggleContextDrawer,
            ),
            ContextPage::Settings => {
                app::context_drawer::context_drawer(app.settings(), Message::ToggleContextDrawer)
                    .title(app.cosmic.context_page.title())
            }
        })
    }
}
