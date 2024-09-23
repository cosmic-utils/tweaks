pub mod cosmic_themes {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CosmicTheme {
        pub name: String,
        pub ron: String,
        pub author: String,
        pub link: String,
    }
}
