use cosmic_ext_tweaks::{app, core};

fn main() -> cosmic::iced::Result {
    let (settings, flags) = core::config::get();
    cosmic::app::run::<app::TweakTool>(settings, flags)
}
