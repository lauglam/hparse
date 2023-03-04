use serde::{Deserialize, Serialize};
use crate::actions::{ActionErrorKind, ActionResult};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FuncAction {
    lang: String,
    value: String,
    description: Option<String>,
    exception: Option<String>,
}

impl FuncAction {
    pub fn new(
        lang: String,
        value: String,
        description: Option<String>,
        exception: Option<String>,
    ) -> FuncAction {
        FuncAction { lang, value, description, exception }
    }

    pub fn act(&self, k: &str) -> ActionResult<String> {
        unimplemented!()
    }
}
