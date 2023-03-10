use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::{actions::{ActionError, ActionErrorKind, ActionResult, CALLBACK}};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CallbackAction {
    tag: String,
    description: Option<String>,
}

impl CallbackAction {
    pub fn execute(&self, s: &str, value: &Value) -> ActionResult<String> {
        match CALLBACK.get().unwrap() {
            Some(callback) => {
                let mut callback = callback.lock().unwrap();
                Ok(String::from(callback(self.tag.clone(), s, value)?))
            }
            None => Err(ActionError(ActionErrorKind::MissingCallbackFunction, None)),
        }
    }
}
