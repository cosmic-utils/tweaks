use serde::{Deserialize, Serialize};

use super::config::ColorScheme;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmicTheme {
    pub name: String,
    pub ron: String,
    pub author: String,
    pub link: String,
}

impl From<CosmicTheme> for ColorScheme {
    fn from(theme: CosmicTheme) -> Self {
        Self {
            name: theme.name,
            path: None,
            link: Some(theme.link),
            author: Some(theme.author),
            theme: ron::from_str(&theme.ron).unwrap(),
        }
    }
}
