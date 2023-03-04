mod reg_action;
mod str_action;
mod choose_action;
mod func_action;
mod attr_action;
mod select_action;
mod any_of_action;

pub use {
    any_of_action::AnyOfAction,
    attr_action::AttrAction,
    choose_action::ChooseAction,
    func_action::FuncAction,
    reg_action::RegAction,
    select_action::SelectAction,
    str_action::StrAction,
};


type ActionResult<T> = Result<T, ActionError>;

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum Action {
    AnyOf(AnyOfAction),
    Attribute(AttrAction),
    Choose(ChooseAction),
    Func(FuncAction),
    Regex(RegAction),
    Select(SelectAction),
    Str(StrAction),
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum ActionErrKind {
    AnyActionAllActionFail,
    ElementNotFound,
    AttributeNotFound,
    RegexNotMatch,
    PatternNotCovered,
    StrEmpty,
    RunFunctionFail,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionError {
    kind: ActionErrKind,
    message: String,
}

impl ActionError {
    pub fn new(kind: ActionErrKind, msg: String) -> ActionError {
        ActionError { kind, message: msg }
    }
}

impl std::fmt::Display for ActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ActionError {}

pub trait ActionErrRes<T> {
    fn res(&self) -> ActionResult<Option<T>>;
}

impl<T> ActionErrRes<T> for Option<ActionError> {
    fn res(&self) -> ActionResult<Option<T>> {
        match self {
            Some(e) => Err(e.clone()),
            None => Ok(None)
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
