use serde::{Deserialize, Serialize};
use crate::actions::{ActionError, ActionErrorKind, ActionResult};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ChooseAction {
    keys: Vec<String>,
    values: Vec<String>,
    description: Option<String>,
    error: ActionError,
}

impl ChooseAction {
    pub fn new(
        keys: Vec<String>,
        values: Vec<String>,
        description: Option<String>,
        error: Option<String>,
    ) -> ChooseAction {
        let error = ActionError::new(
            ActionErrorKind::AnyActionAllActionFail,
            error,
        );

        ChooseAction { keys, values, description, error }
    }

    pub fn act(&self, k: &str) -> ActionResult<String> {
        let idx = self.keys.iter().position(|s| s == k);

        if let Some(idx) = idx {
            Ok(self.values[idx].clone())
        } else {
            Err(self.error.clone())
        }
    }
}
