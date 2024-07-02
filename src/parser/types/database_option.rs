use crate::parser::{Rule, Sql};

#[derive(Debug, Clone)]
pub enum Object {
    CharacterSet(String),
    Collate(String),
    Encryption(String),
}

impl Sql for Object {
    fn as_sql(&self) -> String {
        match self {
            Object::CharacterSet(value) => format!("CHARACTER_SET {value}"),
            Object::Collate(value) => format!("COLLATE {value}"),
            Object::Encryption(value) => format!("ENCRYPTION {value}"),
        }
    }
}

impl Object {
    pub fn from_pair(pair: pest::iterators::Pair<Rule>) -> Option<Object> {
        if let Some(inner_option) = pair.into_inner().next() {
            let key = match inner_option.as_rule() {
                Rule::CHARACTER_SET => Object::CharacterSet,
                Rule::COLLATE => Object::Collate,
                Rule::ENCRYPTION => Object::Encryption,
                _ => return None,
            };
            let value = match inner_option.into_inner().next() {
                Some(value) => value.as_str().trim_matches('`').to_string(),
                None => String::new(),
            };
            Some(key(value))
        } else {
            None
        }
    }
}
