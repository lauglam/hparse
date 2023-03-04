mod regex_action;
mod str_action;
mod choose_action;
mod func_action;
mod attribute_action;
mod select_action;
mod any_of_action;

pub use {
    any_of_action::AnyOfAction,
    attribute_action::AttributeAction,
    choose_action::ChooseAction,
    func_action::FuncAction,
    regex_action::RegexAction,
    select_action::SelectAction,
    str_action::StrAction,
};


type ActionResult<T> = Result<T, ActionError>;

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum Action {
    AnyOf(AnyOfAction),
    Attribute(AttributeAction),
    Choose(ChooseAction),
    Func(FuncAction),
    Regex(RegexAction),
    Select(SelectAction),
    Str(StrAction),
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum ActionErrorKind {
    AnyActionAllActionFail,
    AttributeNotFound,
    PatternNotCovered,
    RunFunctionFail,
    RegexNotMatch,
    ElementNotFound,
    StrEmpty,
}

impl std::fmt::Display for ActionErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionError {
    kind: ActionErrorKind,
    message: Option<String>,
}

impl ActionError {
    pub fn new(kind: ActionErrorKind, msg: Option<String>) -> ActionError {
        ActionError { kind, message: msg }
    }
}

impl std::fmt::Display for ActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.message {
            None => write!(f, "{}", self.kind),
            Some(ref m) => write!(f, "{}: {}", self.kind, m),
        }
    }
}

impl std::error::Error for ActionError {}

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
