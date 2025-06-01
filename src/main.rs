mod app;
mod core;
mod pages;

pub use core::error::Error;
use core::settings;

fn main() -> Result<(), Error> {
    settings::init()?;
    cosmic::app::run::<app::App>(settings::settings(), settings::flags())?;
    Ok(())
}
