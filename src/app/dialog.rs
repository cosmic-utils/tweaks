use crate::pages::layouts::preview::LayoutPreview;

#[derive(Clone, Debug)]
pub enum DialogPage {
    SaveCurrentColorScheme(String),
    CreateSnapshot(String),
    CreateLayout(String, LayoutPreview),
}
