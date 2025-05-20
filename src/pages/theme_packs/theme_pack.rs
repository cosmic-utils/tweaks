use cosmic_ext_config_templates::Schema;
use serde::{Deserialize, Serialize};

use crate::core::resources;
use crate::pages::color_schemes::config::ColorScheme;

#[derive(Debug, Serialize, Deserialize)]
pub struct ColorSchemeConfig {
    pub theme_builder_ron: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PanelConfig {
    pub panel_ron: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemePack {
    // Metadata
    pub name: String,
    pub author: String,
    pub description: String,
    pub version: String,

    // Configurations
    pub color_scheme: ColorSchemeConfig,
    pub layout: Schema,
}

pub fn create_theme_pack(
    name: impl Into<String>,
    author: impl Into<String>,
    description: impl Into<String>,
) -> Result<ThemePack, ron::error::Error> {
    // Get the current color scheme
    let theme_builder = ColorScheme::current_theme();
    let name = name.into();
    let author = author.into();

    // Create a ColorScheme
    let color_scheme = ColorScheme {
        name: name.clone(),
        author: Some(author.clone()),
        path: None,
        link: None,
        theme: theme_builder,
    };

    let color_scheme_ron = ron::to_string(&color_scheme)?;

    // Generate a Schema for the current panel/dock configuration
    let layout = cosmic_ext_config_templates::panel::PanelSchema::generate()
        .map(Schema::Panel)
        .unwrap_or_else(|_| {
            // Fallback to COSMIC layout if we can't generate one
            ron::from_str::<Schema>(resources::COSMIC_LAYOUT).unwrap()
        });

    Ok(ThemePack {
        name: name,
        author: author,
        description: description.into(),
        version: "1.0.0".into(),
        color_scheme: ColorSchemeConfig {
            theme_builder_ron: color_scheme_ron,
        },
        layout,
    })
}
