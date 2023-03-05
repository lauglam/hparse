use serde::{Deserialize, Serialize};
use crate::actions::ActionResult;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FuncAction {
    lang: String,
    code: String,
    description: Option<String>,
    exception: Option<String>,
}

impl FuncAction {
    pub fn new(
        lang: String,
        code: String,
        description: Option<String>,
        exception: Option<String>,
    ) -> FuncAction {
        FuncAction { lang, code, description, exception }
    }

    pub fn act(&self, k: &str) -> ActionResult<String> {
        unimplemented!()
    }
}
