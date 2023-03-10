use serde::{Deserialize, Serialize};
use visdom::Vis;
use crate::actions::{ActionError, ActionErrorKind, ActionResult, Variable};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SelectAction {
    selector: Variable<String>,
    description: Option<String>,
    exception: Option<String>,
}

impl SelectAction {
    pub fn execute(&self, s: &str) -> ActionResult<String> {
        let r = Vis::load(s).unwrap();
        let r = r.children("");

        self.selector.using(
            &mut |s| {
                let h = r.find(s).outer_html();

                match h.is_empty() {
                    true => Err(ActionError(
                        ActionErrorKind::ElementNotFound,
                        self.exception.clone(),
                    )),
                    false => Ok(h),
                }
            },
            &mut |a| {
                for s in a {
                    let h = r.find(s).outer_html();
                    if !h.is_empty() {
                        return Ok(h);
                    }
                }

                Err(ActionError(ActionErrorKind::ElementNotFound, self.exception.clone()))
            },
        )
    }
}
