use anyhow::Result;

use cosmic::cosmic_theme::ThemeBuilder;
use uuid::Uuid;

use crate::app::pages::color_schemes::{models::Source, storage::TempColorScheme};

#[derive(serde::Deserialize)]
pub struct RemoteTheme {
    #[allow(unused)]
    id: u32,
    uuid: Uuid,
    pub name: String,
    ron: String,
    author: Option<String>,
    link: Option<String>,
    downloads: u64,
    pub created: String,
    updated: String,
}

pub async fn download_themes() -> Result<Vec<TempColorScheme>> {
    let themes: Vec<RemoteTheme> =
        reqwest::get("https://cosmic-themes.org/api/themes/?limit=50000")
            .await?
            .json()
            .await?;

    Ok(themes
        .into_iter()
        .filter_map(|t| {
            let name = t.name.clone();
            match TempColorScheme::try_from(t) {
                Ok(scheme) => Some(scheme),
                Err(err) => {
                    log::error!("skipping theme {name}: {err}");
                    None
                }
            }
        })
        .collect())
}

impl TryFrom<RemoteTheme> for TempColorScheme {
    type Error = anyhow::Error;

    fn try_from(value: RemoteTheme) -> Result<Self, Self::Error> {
        let builder: ThemeBuilder = ron::from_str(&value.ron)
            .map_err(|err| anyhow::anyhow!("invalid theme format for {}: {}", value.name, err))?;

        Ok(TempColorScheme {
            id: value.uuid,
            name: value.name,
            theme_builder: builder.clone(),
            theme: builder.build().into(),
            author: value.author.filter(|x| !x.is_empty()),
            link: value.link.filter(|x| !x.is_empty()),
            downloads: Some(value.downloads),
            created: Some(chrono::DateTime::parse_from_rfc3339(&value.created)?.timestamp_millis()),
            updated: Some(chrono::DateTime::parse_from_rfc3339(&value.updated)?.timestamp_millis()),
            source: Some(Source::CosmicThemesOrg),
            path: None,
        })
    }
}
