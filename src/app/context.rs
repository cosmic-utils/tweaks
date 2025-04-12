use crate::fl;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ContextPage {
    Settings,
    About,
}

impl ContextPage {
    pub fn title(&self) -> String {
        match self {
            Self::About => fl!("about"),
            Self::Settings => fl!("settings"),
        }
    }
}
