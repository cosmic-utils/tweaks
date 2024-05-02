use app::TweakTool;
mod app;
mod core;
mod pages;

fn main() -> cosmic::iced::Result {
    let settings = core::config::get();
    cosmic::app::run::<TweakTool>(settings, ())
}
