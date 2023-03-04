use serde::{Deserialize, Serialize};
use visdom::Vis;
use crate::actions::{ActionError, ActionErrKind, ActionErrRes, ActionResult};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StrAction {
    description: Option<String>,
    error: Option<ActionError>,
}

impl StrAction {
    pub fn new(
        description: Option<String>,
        error: Option<String>,
    ) -> StrAction {
        let error = if let Some(msg) = error {
            Some(ActionError::new(
                ActionErrKind::StrEmpty,
                msg,
            ))
        } else {
            None
        };

        StrAction { description, error }
    }

    pub fn act(&self, s: &str) -> ActionResult<Option<String>> {
        let r = Vis::load(s).unwrap();
        let t = r.text();

        if t.is_empty() {
            self.error.res()
        } else {
            Ok(Some(t))
        }
    }
}
