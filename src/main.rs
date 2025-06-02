mod app;

pub use app::core::error::Error;
use app::core::settings;

fn main() -> Result<(), Error> {
    settings::init()?;
    cosmic::app::run::<app::App>(settings::settings(), settings::flags())?;
    Ok(())
}
