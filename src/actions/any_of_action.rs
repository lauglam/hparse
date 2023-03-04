use std::ops::Deref;
use crate::actions::{Action, ActionErrKind, ActionError, ActionErrRes, ActionResult};

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct AnyOfAction {
    actions: Box<Vec<Action>>,
    description: Option<String>,
    error: Option<ActionError>,
}

impl AnyOfAction {
    pub fn new(
        actions: Vec<Action>,
        description: Option<String>,
        error: Option<String>,
    ) -> AnyOfAction {
        let error = if let Some(msg) = error {
            Some(ActionError::new(
                ActionErrKind::AnyActionAllActionFail,
                msg,
            ))
        } else {
            None
        };

        AnyOfAction { actions: Box::new(actions), description, error }
    }

    pub fn act(&self, s: &str) -> ActionResult<Option<String>> {
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
                if let Some(s) = s {
                    // the child action's return can be `None` or `Err`
                    // just return `Some` res
                    return Ok(Some(s));
                }
            }
        }

        self.error.res()
    }
}
