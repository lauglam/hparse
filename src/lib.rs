mod actions;
mod hparse;

pub use hparse::{HParse, ParseFile};

use crate::actions::ActionError;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    MissingProperties,
    ParseParseFileFail(serde_yaml::Error),
    ActionError(ActionError),
}

impl From<ActionError> for ParseError {
    fn from(value: ActionError) -> Self {
        ParseError::ActionError(value)
    }
}

impl From<serde_yaml::Error> for ParseError {
    fn from(value: serde_yaml::Error) -> Self {
        ParseError::ParseParseFileFail(value)
    }
}
