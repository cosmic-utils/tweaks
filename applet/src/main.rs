// SPDX-License-Identifier: GPL-3.0-only

use app::TweaksApplet;
mod app;
mod core;

fn main() -> cosmic::iced::Result {
    core::config::init();
    cosmic::applet::run::<TweaksApplet>(true, ())
}
