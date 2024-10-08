use std::fmt::Display;


#[derive(Debug, thiserror::Error)]
pub enum NotflyError {
    #[allow(unused)]
    RowNotFound,
}

impl Display for NotflyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotflyError::RowNotFound => write!(f, "Row not found"),
        }
    }
}