use serde::{Deserialize, Serialize};
use crate::actions::{ActionError, ActionErrorKind, ActionResult};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FuncAction {
    lang: String,
    value: String,
    description: Option<String>,
    error: ActionError,
}

impl FuncAction {
    pub fn new(
        lang: String,
        value: String,
        description: Option<String>,
        error: Option<String>,
    ) -> FuncAction {
        let error = ActionError::new(
            ActionErrorKind::AnyActionAllActionFail,
            error,
        );

        FuncAction { lang, value, description, error }
    }

    pub fn act(&self, k: &str) -> ActionResult<String> {
        unimplemented!()
    }
}
