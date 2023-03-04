use serde::{Deserialize, Serialize};
use visdom::Vis;
use crate::actions::{ActionError, ActionErrorKind, ActionResult};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StrAction {
    description: Option<String>,
    error: ActionError,
}

impl StrAction {
    pub fn new(
        description: Option<String>,
        error: Option<String>,
    ) -> StrAction {
        let error = ActionError::new(
            ActionErrorKind::AnyActionAllActionFail,
            error,
        );

        StrAction { description, error }
    }

    pub fn act(&self, s: &str) -> ActionResult<String> {
        let r = Vis::load(s).unwrap();
        let t = r.text();

        match t.is_empty() {
            true => Err(self.error.clone()),
            false => Ok(t)
        }
    }
}
