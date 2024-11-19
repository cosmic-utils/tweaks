mod app;
mod core;
mod pages;

use app::settings;

fn main() -> cosmic::iced::Result {
    settings::init();
    cosmic::app::run::<app::TweakTool>(settings::settings(), settings::flags())
}
