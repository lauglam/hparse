use serde::{Deserialize, Serialize};
use crate::actions::{ActionError, ActionErrKind, ActionErrRes, ActionResult};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ChooseAction {
    keys: Vec<String>,
    values: Vec<String>,
    description: Option<String>,
    error: Option<ActionError>,
}

impl ChooseAction {
    pub fn new(
        keys: Vec<String>,
        values: Vec<String>,
        description: Option<String>,
        error: Option<String>,
    ) -> ChooseAction {
        let error = if let Some(msg) = error {
            Some(ActionError::new(
                ActionErrKind::PatternNotCovered,
                msg,
            ))
        } else {
            None
        };

        ChooseAction { keys, values, description, error }
    }

    pub fn act(&self, k: &str) -> ActionResult<Option<String>> {
        let idx = self.keys.iter().position(|s| s == k);

        if let Some(idx) = idx {
            Ok(Some(self.values[idx].clone()))
        } else {
            self.error.res()
        }
    }
}
