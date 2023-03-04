use serde::{Deserialize, Serialize};
use regex::Regex;
use crate::actions::{ActionError, ActionErrKind, ActionErrRes, ActionResult, Variable};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RegAction {
    regex: Variable<String>,
    group: usize,
    description: Option<String>,
    error: Option<ActionError>,
}

impl RegAction {
    pub fn new(
        regex: Variable<String>,
        group: usize,
        description: Option<String>,
        error: Option<String>,
    ) -> RegAction {
        let error = if let Some(msg) = error {
            Some(ActionError::new(
                ActionErrKind::RegexNotMatch,
                msg,
            ))
        } else {
            None
        };

        RegAction { regex, group, description, error }
    }

    pub fn act(&self, s: &str) -> ActionResult<Option<String>> {
        self.regex.using(
            &mut |reg| {
                let r = Regex::new(reg).unwrap();

                if let Some(cap) = r.captures(s) {
                    Ok(Some(String::from(&cap[self.group])))
                } else {
                    self.error.res()
                }
            },
            &mut |a| {
                for reg in a {
                    let r = Regex::new(reg).unwrap();

                    if let Some(cap) = r.captures(s) {
                        return Ok(Some(String::from(&cap[self.group])));
                    }
                }
                self.error.res()
            },
        )
    }
}
