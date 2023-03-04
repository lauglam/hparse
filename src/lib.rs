pub mod actions;

use crate::actions::Action;

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParseFile {
    kind: ParseKind,
    name: String,
    actions: Option<Vec<Action>>,
    properties: Box<Vec<ParseFile>>,
    use_parent: bool,

    description: Option<String>,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum ParseKind {
    Object,
    Array,
    String,
    Number,
}

impl ParseFile {
    pub fn new(
        kind: ParseKind,
        name: String,
        actions: Option<Vec<Action>>,
        properties: Vec<ParseFile>,
        use_parent: bool,
        description: Option<String>,
    ) -> ParseFile {
        let properties = Box::new(properties);

        ParseFile {
            kind,
            name,
            actions,
            properties,
            use_parent,

            description,
        }
    }

    pub fn execute(&self, s: &str, parent: Option<String>) {

    }
}
