mod app;
mod core;
mod pages;

pub use app::error::Error;
use app::settings;

fn main() -> cosmic::iced::Result {
    settings::init();
    cosmic::app::run::<app::App>(settings::settings(), settings::flags())
}
