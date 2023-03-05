use std::sync::Mutex;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use crate::actions::{ActionError, ActionErrorKind, ActionResult};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CallbackAction {
    tag: String,
    description: Option<String>,
}

impl CallbackAction {
    pub fn new(
        tag: String,
        description: Option<String>,
    ) -> CallbackAction {
        CallbackAction { tag, description }
    }

    pub fn act(&self, s: &str) -> ActionResult<String> {
        match CALLBACK.lock().unwrap().as_ref() {
            Some(call) => Ok(String::from(&call(self.tag.clone(), String::from(s)))),
            None => Err(ActionError::new(ActionErrorKind::MissingCallbackFunction, None)),
        }
    }
}

lazy_static! {
    pub static ref CALLBACK: Mutex<Option<Callback>> = Mutex::new(None);
}

pub type Callback = Box<dyn FnMut(String, String) -> String + Sync + Send>;
