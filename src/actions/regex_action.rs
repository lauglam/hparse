use serde::{Deserialize, Serialize};
use regex::Regex;
use crate::actions::{ActionError, ActionErrorKind, ActionResult, Variable};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RegexAction {
    regex: Variable<String>,
    group: usize,
    description: Option<String>,
    exception: Option<String>,
}

impl RegexAction {
    pub fn execute(&self, s: &str) -> ActionResult<String> {
        match self.regex {
            Variable::Single(ref reg) => {
                let r = Regex::new(reg).unwrap();

                if let Some(cap) = r.captures(s) {
                    Ok(String::from(&cap[self.group]))
                } else {
                    Err(ActionError(ActionErrorKind::RegexNotMatch, self.exception.clone()))
                }
            }
            Variable::AnyOf(ref regs) => {
                for reg in regs {
                    let r = Regex::new(reg).unwrap();

                    if let Some(cap) = r.captures(s) {
                        return Ok(String::from(&cap[self.group]));
                    }
                }

                Err(ActionError(ActionErrorKind::RegexNotMatch, self.exception.clone()))
            }
        }
    }
}
