use serde::{Deserialize, Serialize};
use visdom::Vis;
use crate::actions::{ActionError, ActionErrKind, ActionErrRes, ActionResult, Variable};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AttrAction {
    attr: Variable<String>,
    description: Option<String>,
    error: Option<ActionError>,
}

impl AttrAction {
    pub fn new(
        attr: Variable<String>,
        description: Option<String>,
        error: Option<String>,
    ) -> AttrAction {
        let error = if let Some(msg) = error {
            Some(ActionError::new(
                ActionErrKind::AttributeNotFound,
                msg,
            ))
        } else {
            None
        };

        AttrAction { attr, description, error }
    }

    pub fn act(&self, s: &str) -> ActionResult<Option<String>> {
        let r = Vis::load(s).unwrap();

        self.attr.using(
            &mut |s| {
                if let Some(attr) = r.attr(s) {
                    Ok(Some(attr.to_string()))
                } else {
                    self.error.res()
                }
            },
            &mut |a| {
                for s in a {
                    if let Some(attr) = r.attr(s) {
                        return Ok(Some(attr.to_string()));
                    }
                }
                self.error.res()
            },
        )
    }
}
