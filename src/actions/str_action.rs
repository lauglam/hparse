use serde::{Deserialize, Serialize};
use visdom::Vis;
use crate::actions::{ActionError, ActionErrorKind, ActionResult};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StrAction {
    description: Option<String>,
    exception: Option<String>,
}

impl StrAction {
    pub fn execute(&self, s: &str) -> ActionResult<String> {
        let r = Vis::load(s).unwrap();
        let r = r.children("");
        let t = r.text();

        match t.is_empty() {
            true => Err(ActionError(
                ActionErrorKind::StrEmpty,
                self.exception.clone(),
            )),
            false => Ok(t)
        }
    }
}
