pub use app::core::error::Error;
use app::core::settings;

#[allow(unused_imports)]
#[macro_use]
extern crate log;

#[macro_use]
pub mod localize;

mod app;

fn main() -> Result<(), Error> {
    settings::init()?;
    cosmic::app::run::<app::App>(settings::settings(), settings::flags())?;
    Ok(())
}
