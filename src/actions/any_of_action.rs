use crate::actions::{Action, ActionErrorKind, ActionError, ActionResult};

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct AnyOfAction {
    actions: Box<Vec<Action>>,
    description: Option<String>,
    exception: Option<String>,
}

impl AnyOfAction {
    pub fn new(
        actions: Vec<Action>,
        description: Option<String>,
        exception: Option<String>,
    ) -> AnyOfAction {
        AnyOfAction { actions: Box::new(actions), description, exception }
    }

    pub fn act(&self, s: &str) -> ActionResult<String> {
        for action in self.actions.as_ref() {
            if let Ok(s) = action.act(s) {
                return Ok(s);
            }
            // ignore `actions` item error
        }

        Err(ActionError::new(ActionErrorKind::AnyActionAllActionFail, self.exception.clone()))
    }
}
