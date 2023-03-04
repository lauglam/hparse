use std::ops::Deref;
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
        for action in self.actions.deref() {
            let res = match action {
                Action::AnyOf(a) => a.act(s),
                Action::Attribute(a) => a.act(s),
                Action::Choose(a) => a.act(s),
                Action::Func(a) => a.act(s),
                Action::Regex(a) => a.act(s),
                Action::Select(a) => a.act(s),
                Action::Str(a) => a.act(s),
            };

            if let Ok(s) = res {
                // ignore `actions` item error
                return Ok(s);
            }
        }

        Err(self.error.clone())
    }
}
