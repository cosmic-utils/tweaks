mod app;
mod core;
mod pages;
mod settings;

fn main() -> cosmic::iced::Result {
    let (settings, flags) = core::config::get();
    cosmic::app::run::<app::TweakTool>(settings, flags)
}
