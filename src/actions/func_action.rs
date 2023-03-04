use serde::{Deserialize, Serialize};
use crate::actions::{ActionError, ActionErrKind, ActionResult};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FuncAction {
    lang: String,
    value: String,
    description: Option<String>,
    error: Option<ActionError>,
}

impl FuncAction {
    pub fn new(
        lang: String,
        value: String,
        description: Option<String>,
        error: Option<String>,
    ) -> FuncAction {
        let error = if let Some(msg) = error {
            Some(ActionError::new(
                ActionErrKind::RunFunctionFail,
                msg,
            ))
        } else {
            None
        };

        FuncAction { lang, value, description, error }
    }

    pub fn act(&self, k: &str) -> ActionResult<Option<String>> {
        unimplemented!()
    }
}
