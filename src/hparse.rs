use std::cell::RefMut;
use std::sync::Mutex;
use serde_json::{Map, Value};
use crate::{ParseError, ParseResult, actions::{Action, Callback, CALLBACK}};

pub struct HParse {
    cfg: ParseFile,
}

impl HParse {
    pub fn new(cfg: &str) -> ParseResult<HParse> {
        let cfg = serde_yaml::from_str::<ParseFile>(cfg)?;
        Ok(HParse { cfg })
    }

    pub fn json(&self, html: &str, callback: Option<Callback>) -> ParseResult<Value> {
        _ = CALLBACK.set(callback.map(|f| Mutex::new(f)));
        self.cfg.json(html)
    }
}

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
    description: Option<String>,
    // default `false`
    #[serde(default)]
    nullable: bool,
}

impl ParseFile {
    pub fn json(&self, s: &str) -> ParseResult<Value> {
        let mut res = self.new_value();

        let pre = self.actions(s, &mut res);

        // Nullable is `true` and error occurs
        if self.nullable && pre.is_err() {
            return Ok(Value::Null);
        }

        // Boolean
        if self.kind == ParseKind::Boolean {
            // `kind` is `Boolean`
            return Ok(Value::Bool(pre.is_ok()));
        }

        let pre = pre?;

        // String
        if self.kind == ParseKind::String {
            return Ok(Value::String(pre));
        }

        match self.properties.as_ref() {
            Some(props) => {
                for prop in props {
                    let value = prop.json(&pre)?;
                    self.insert(&mut res, prop.name.clone(), value);
                }

                Ok(res)
            }
            // kind not `string`, but `properties` is `None`
            None => Err(ParseError::MissingProperties),
        }
    }

    fn actions(&self, s: &str, value: &Value) -> ParseResult<String> {
        let mut res = String::from(s);
        if let Some(ref actions) = self.actions {
            for action in actions {
                res = action.execute(&res, value)?;
            }
        }
        Ok(res)
    }

    fn insert(&self, res: &mut Value, key: String, value: Value) {
        match self.kind {
            ParseKind::Object => {
                let s = res.as_object_mut().unwrap();
                s.insert(key, value);
            }
            ParseKind::Array => {
                let s = res.as_array_mut().unwrap();
                s.push(value);
            }
            _ => unreachable!(),
        }
    }

    fn new_value(&self) -> Value {
        match self.kind {
            ParseKind::Object => Value::Object(Map::new()),
            ParseKind::Array => Value::Array(Vec::new()),
            ParseKind::String => Value::String(String::new()),
            ParseKind::Boolean => Value::Bool(false),
        }
    }
}
