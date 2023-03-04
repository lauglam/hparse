use serde::{Deserialize, Serialize};
use visdom::Vis;
use crate::actions::{ActionError, ActionErrKind, ActionErrRes, ActionResult, Variable};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SelectAction {
    selector: Variable<String>,
    description: Option<String>,
    error: Option<ActionError>,
}

impl SelectAction {
    pub fn new(
        selector: Variable<String>,
        description: Option<String>,
        error: Option<String>,
    ) -> SelectAction {
        let error = if let Some(msg) = error {
            Some(ActionError::new(
                ActionErrKind::ElementNotFound,
                msg,
            ))
        } else {
            None
        };

        SelectAction { selector, description, error }
    }

    pub fn act(&self, s: &str) -> ActionResult<Option<String>> {
        let r = Vis::load(s).unwrap();

        self.selector.using(
            &mut |s| {
                let h = r.find(s).outer_html();

                if h.is_empty() {
                    self.error.res()
                } else {
                    Ok(Some(h))
                }
            },
            &mut |a| {
                for s in a {
                    let h = r.find(s).outer_html();
                    if !h.is_empty() {
                        return Ok(Some(h));
                    }
                }
                self.error.res()
            },
        )
    }
}
