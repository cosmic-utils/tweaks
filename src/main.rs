pub use app::core::error::Error;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::app::flags::Flags;

#[allow(unused_imports)]
#[macro_use]
extern crate log;

#[macro_use]
pub mod i18n;

mod app;

fn main() -> Result<(), Error> {
    // Get the system's preferred languages.
    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();

    // Enable localizations to be applied.
    i18n::init(&requested_languages);

    // Initialize tracing for logging and debugging.
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("tweaks=info")),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize desktop layouts.
    crate::app::pages::layouts::Layouts::init()?;

    // Initialize app flags.
    let flags = Flags {
        handler: crate::app::core::config::TweaksConfig::config(),
        config: crate::app::core::config::TweaksConfig::new(),
    };

    // Initialize app settings.
    let settings = cosmic::app::Settings::default().size_limits(
        cosmic::iced::Limits::NONE
            .min_width(360.0)
            .min_height(180.0),
    );

    cosmic::app::run::<app::App>(settings, flags).map_err(Error::Iced)
}
