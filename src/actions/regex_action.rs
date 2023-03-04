use serde::{Deserialize, Serialize};
use regex::Regex;
use crate::actions::{ActionError, ActionErrorKind, ActionResult, Variable};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RegexAction {
    regex: Variable<String>,
    group: usize,
    description: Option<String>,
    error: ActionError,
}

impl RegexAction {
    pub fn new(
        regex: Variable<String>,
        group: usize,
        description: Option<String>,
        error: Option<String>,
    ) -> RegexAction {
        let error = ActionError::new(
            ActionErrorKind::AnyActionAllActionFail,
            error,
        );

        RegexAction { regex, group, description, error }
    }

    pub fn act(&self, s: &str) -> ActionResult<String> {
        self.regex.using(
            &mut |reg| {
                let r = Regex::new(reg).unwrap();

                if let Some(cap) = r.captures(s) {
                    Ok(String::from(&cap[self.group]))
                } else {
                    Err(self.error.clone())
                }
            },
            &mut |a| {
                for reg in a {
                    let r = Regex::new(reg).unwrap();

                    if let Some(cap) = r.captures(s) {
                        return Ok(String::from(&cap[self.group]));
                    }
                }
                Err(self.error.clone())
            },
        )
    }
}
