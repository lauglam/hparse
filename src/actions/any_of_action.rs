use serde_json::Value;
use crate::{actions::{Action, ActionErrorKind, ActionError, ActionResult}};

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct AnyOfAction {
    actions: Box<Vec<Action>>,
    description: Option<String>,
    exception: Option<String>,
}

impl AnyOfAction {
    pub fn execute(&self, s: &str, value: &Value) -> ActionResult<String> {
        for action in self.actions.as_ref() {
            if let Ok(s) = action.execute(s, value) {
                return Ok(s);
            }
            // ignore `actions` item error
        }

        Err(ActionError(ActionErrorKind::AnyActionAllActionFail, self.exception.clone()))
    }
}
