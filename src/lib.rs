#![feature(is_some_and)]

pub mod actions;

use crate::actions::{Action, ActionError};

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum ParseKind {
    Object,
    Array,
    String,
    Boolean,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParseFile {
    kind: ParseKind,
    name: String,
    // `None` if input string not need to pre-process
    actions: Option<Vec<Action>>,
    // `None` if kind is string or number
    properties: Box<Option<Vec<ParseFile>>>,
    // default `false`
    use_parent: Option<bool>,
    // default `false`
    nullable: Option<bool>,

    description: Option<String>,
}

impl ParseFile {
    pub fn new(
        kind: ParseKind,
        name: String,
        actions: Option<Vec<Action>>,
        properties: Option<Vec<ParseFile>>,
        use_parent: Option<bool>,
        nullable: Option<bool>,
        description: Option<String>,
    ) -> ParseFile {
        let properties = Box::new(properties);

        ParseFile {
            kind,
            name,
            actions,
            properties,
            use_parent,
            nullable,

            description,
        }
    }

    pub fn json(&self, s: &str, parent: Option<&str>) -> ParseResult<String> {
        let pre_process = self.use_actions(self.use_parent(s, parent));

        if self.nullable.is_some_and(|b| b && pre_process.is_err()) {
            // `nullable` is `true`
            return Ok(String::new());
        }

        if self.kind == ParseKind::Boolean {
            // `kind` is `Boolean`
            return Ok(format!(r#""{}":"{}""#, self.name, pre_process.is_ok()));
        }

        let pre_process = pre_process?;

        if self.kind == ParseKind::String {
            return Ok(format!(r#""{}":"{}""#, self.name, pre_process));
        }

        match self.properties.as_ref() {
            Some(props) => {
                let mut res = Vec::new();
                for prop in props {
                    res.push(prop.json(s, parent)?);
                }

                let res = res.join(",");

                // root:
                // {
                //   "gid": "xxx",
                //   "token": "xxx",
                //  }
                //
                // not root:
                // "category": {
                //               "gid": "xxx",
                //               "token": "xxx",
                //             }
                let is_root = parent.is_none();
                match self.kind {
                    ParseKind::Object => Ok(
                        if is_root {
                            format!(r"{{{}}}", res)
                        } else {
                            format!(r#""{}":{{{}}}"#, self.name, res)
                        }
                    ),
                    ParseKind::Array => Ok(
                        if is_root {
                            format!("[{}]", res)
                        } else {
                            format!(r#""{}":[{}]"#, self.name, res)
                        }
                    ),
                    _ => unreachable!(),
                }
            }
            // kind not `string`, but `properties` is `None`
            None => Err(ParseError::MissingProperties),
        }
    }

    fn use_actions(&self, s: &str) -> ParseResult<String> {
        let mut res = String::from(s);

        if let Some(ref actions) = self.actions {
            for action in actions {
                res = action.act(&res)?;
            }
        }

        Ok(res)
    }

    fn use_parent<'a>(&'a self, s: &'a str, parent: Option<&'a str>) -> &'a str {
        if self.use_parent.is_some_and(|b| b) {
            if let Some(p) = parent {
                return p;
            }
        }
        s
    }
}

pub type ParseResult<T> = Result<T, ParseError>;

pub enum ParseError {
    MissingProperties,
    ParseParseFileFail(serde_yaml::Error),
    ActionError(ActionError),
}

impl From<ActionError> for ParseError {
    fn from(value: ActionError) -> Self {
        ParseError::ActionError(value)
    }
}

impl From<serde_yaml::Error> for ParseError {
    fn from(value: serde_yaml::Error) -> Self {
        ParseError::ParseParseFileFail(value)
    }
}
