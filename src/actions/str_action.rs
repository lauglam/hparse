use serde::{Deserialize, Serialize};
use visdom::Vis;
use crate::actions::{ActionError, ActionErrorKind, ActionResult};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StrAction {
    description: Option<String>,
    exception: Option<String>,
}

impl StrAction {
    pub fn new(
        description: Option<String>,
        exception: Option<String>,
    ) -> StrAction {
        StrAction { description, exception }
    }

    pub fn act(&self, s: &str) -> ActionResult<String> {
        let r = Vis::load(s).unwrap();
        let t = r.text();

        match t.is_empty() {
            true => Err(ActionError::new(
                ActionErrorKind::StrEmpty,
                self.exception.clone(),
            )),
            false => Ok(t)
        }
    }
}
