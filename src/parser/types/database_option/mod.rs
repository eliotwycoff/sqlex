use crate::parser::{Rule, Sql};

#[derive(Debug, Clone)]
pub enum DatabaseOption {
    CharacterSet(String),
    Collate(String),
    Encryption(String),
}

impl Sql for DatabaseOption {
    fn as_sql(&self) -> String {
        match self {
            DatabaseOption::CharacterSet(value) => format!("CHARACTER_SET {value}"),
            DatabaseOption::Collate(value) => format!("COLLATE {value}"),
            DatabaseOption::Encryption(value) => format!("ENCRYPTION {value}"),
        }
    }
}

impl DatabaseOption {
    pub fn from_pair(pair: pest::iterators::Pair<Rule>) -> Option<DatabaseOption> {
        if let Some(inner_option) = pair.into_inner().next() {
            let key = match inner_option.as_rule() {
                Rule::CHARACTER_SET => DatabaseOption::CharacterSet,
                Rule::COLLATE => DatabaseOption::Collate,
                Rule::ENCRYPTION => DatabaseOption::Encryption,
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
