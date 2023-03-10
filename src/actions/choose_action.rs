use serde::{Deserialize, Serialize};
use crate::actions::{ActionError, ActionErrorKind, ActionResult};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ChooseAction {
    keys: Vec<String>,
    values: Vec<String>,
    description: Option<String>,
    exception: Option<String>,
}

impl ChooseAction {
    pub fn execute(&self, k: &str) -> ActionResult<String> {
        let idx = self.keys.iter().position(|s| s == k);

        if let Some(idx) = idx {
            Ok(self.values[idx].clone())
        } else {
            Err(ActionError(ActionErrorKind::PatternNotCovered, self.exception.clone()))
        }
    }
}
