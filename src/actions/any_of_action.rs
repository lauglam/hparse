use crate::actions::{Action, ActionErrorKind, ActionError, ActionResult};

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct AnyOfAction {
    actions: Box<Vec<Action>>,
    description: Option<String>,
    error: ActionError,
}

impl AnyOfAction {
    pub fn new(
        actions: Vec<Action>,
        description: Option<String>,
        error: Option<String>,
    ) -> AnyOfAction {
        let error = ActionError::new(
            ActionErrorKind::AnyActionAllActionFail,
            error,
        );

        AnyOfAction { actions: Box::new(actions), description, error }
    }

    pub fn act(&self, s: &str) -> ActionResult<String> {
        for action in self.actions.as_ref() {
            if let Ok(s) = action.act(s) {
                return Ok(s);
            }
            // ignore `actions` item error
        }

        Err(self.error.clone())
    }
}
