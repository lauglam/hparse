use serde::{Deserialize, Serialize};
use visdom::Vis;
use crate::actions::{ActionError, ActionErrorKind, ActionResult, Variable};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AttributeAction {
    attr: Variable<String>,
    description: Option<String>,
    exception: Option<String>,
}

impl AttributeAction {
    pub fn execute(&self, s: &str) -> ActionResult<String> {
        let r = Vis::load(s).unwrap();
        let r = r.children("");

        self.attr.using(
            &mut |s| {
                if let Some(attr) = r.attr(s) {
                    Ok(attr.to_string())
                } else {
                    Err(ActionError(ActionErrorKind::AttributeNotFound, self.exception.clone()))
                }
            },
            &mut |a| {
                for s in a {
                    if let Some(attr) = r.attr(s) {
                        return Ok(attr.to_string());
                    }
                }

                Err(ActionError(ActionErrorKind::AttributeNotFound, self.exception.clone()))
            },
        )
    }
}
