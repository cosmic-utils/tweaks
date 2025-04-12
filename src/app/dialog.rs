#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DialogPage {
    SaveCurrentColorScheme(String),
    CreateSnapshot(String),
}
