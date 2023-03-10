mod regex_action;
mod str_action;
mod choose_action;
mod callback_action;
mod attribute_action;
mod select_action;
mod any_of_action;

pub use {
    any_of_action::AnyOfAction,
    attribute_action::AttributeAction,
    choose_action::ChooseAction,
    callback_action::CallbackAction,
    regex_action::RegexAction,
    select_action::SelectAction,
    str_action::StrAction,
};

use std::sync::Mutex;
use once_cell::sync::OnceCell;
use serde_json::Value;

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum Action {
    AnyOf(AnyOfAction),
    Attribute(AttributeAction),
    Choose(ChooseAction),
    Callback(CallbackAction),
    Regex(RegexAction),
    Select(SelectAction),
    Str(StrAction),
}

impl Action {
    pub fn execute(&self, s: &str, value: &Value) -> ActionResult<String> {
        match self {
            Action::AnyOf(a) => a.execute(s, value),
            Action::Attribute(a) => a.execute(s),
            Action::Choose(a) => a.execute(s),
            Action::Callback(a) => a.execute(s, value),
            Action::Regex(a) => a.execute(s),
            Action::Select(a) => a.execute(s),
            Action::Str(a) => a.execute(s),
        }
    }
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum Variable<T> {
    Single(T),
    AnyOf(Vec<T>),
}

impl<T> Variable<T> {
    pub fn using<S, A, R>(&self, single: &mut S, any_of: &mut A) -> ActionResult<R>
        where S: FnMut(&T) -> ActionResult<R>,
              A: FnMut(&Vec<T>) -> ActionResult<R>,
    {
        match self {
            Variable::Single(s) => single(s),
            Variable::AnyOf(a) => any_of(a),
        }
    }
}

impl<T> From<T> for Variable<T> {
    fn from(value: T) -> Self {
        Variable::Single(value)
    }
}

impl<T> From<Vec<T>> for Variable<T> {
    fn from(value: Vec<T>) -> Self {
        Variable::AnyOf(value)
    }
}

pub type ActionResult<T> = Result<T, ActionError>;

pub type BoxDynError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Callback = Box<dyn FnMut(String, &str, &Value) -> Result<String, BoxDynError> + Send + Sync + 'static>;

pub static CALLBACK: OnceCell<Option<Mutex<Callback>>> = OnceCell::new();

#[derive(Debug)]
pub enum ActionErrorKind {
    AnyActionAllActionFail,
    AttributeNotFound,
    MissingCallbackFunction,
    CallbackFunctionError(BoxDynError),
    PatternNotCovered,
    RegexNotMatch,
    ElementNotFound,
    StrEmpty,
}

impl std::fmt::Display for ActionErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub struct ActionError(ActionErrorKind, Option<String>);

impl std::fmt::Display for ActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            ActionErrorKind::CallbackFunctionError(ref e) => e.fmt(f),
            _ => match self.1 {
                Some(ref m) => write!(f, "{}: {}", self.0, m),
                None => write!(f, "{}", self.0),
            }
        }
    }
}

impl std::error::Error for ActionError {}

impl From<BoxDynError> for ActionError {
    fn from(value: BoxDynError) -> Self {
        ActionError(ActionErrorKind::CallbackFunctionError(value), None)
    }
}
