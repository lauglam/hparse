use serde::{Deserialize, Serialize};
use visdom::Vis;
use crate::actions::{ActionError, ActionErrorKind, ActionResult, Variable};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SelectAction {
    selector: Variable<String>,
    description: Option<String>,
    error: ActionError,
}

impl SelectAction {
    pub fn new(
        selector: Variable<String>,
        description: Option<String>,
        error: Option<String>,
    ) -> SelectAction {
        let error = ActionError::new(
            ActionErrorKind::AnyActionAllActionFail,
            error,
        );

        SelectAction { selector, description, error }
    }

    pub fn act(&self, s: &str) -> ActionResult<String> {
        let r = Vis::load(s).unwrap();

        self.selector.using(
            &mut |s| {
                let h = r.find(s).outer_html();

                match h.is_empty() {
                    true => Err(self.error.clone()),
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
                Err(self.error.clone())
            },
        )
    }
}
