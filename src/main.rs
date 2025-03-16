mod app;
mod core;
mod pages;

use app::settings;

fn main() -> cosmic::iced::Result {
    settings::init();
    cosmic::app::run::<app::App>(settings::settings(), settings::flags())
}
