use cosmic::cosmic_config::Config;

use crate::app::core::config::TweaksConfig;

#[derive(Clone, Debug)]
pub struct Flags {
    pub handler: Config,
    pub config: TweaksConfig,
}
