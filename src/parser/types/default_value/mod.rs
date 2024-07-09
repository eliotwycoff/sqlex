use crate::parser::{types::TEMPLATES, Rule, Sql};
use pest::iterators::Pair;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum DefaultValue {
    Null,
    CurrentTimestamp { value: Option<u32> },
    Text { value: String },
    Number { value: String },
}

impl From<Pair<'_, Rule>> for DefaultValue {
    fn from(pair: Pair<'_, Rule>) -> Self {
        match pair
            .as_str()
            .to_ascii_uppercase()
            .split_ascii_whitespace()
            .next()
            .expect("keyword")
        {
            "NULL" => Self::Null,
            "CURRENT_TIMESTAMP" => Self::CurrentTimestamp {
                value: pair
                    .into_inner()
                    .next()
                    .map(|p| p.as_str().parse::<u32>().unwrap()),
            },
            _ => {
                let inner = pair.into_inner().next().expect("inner pair");

                match inner.as_rule() {
                    Rule::STRING_LITERAL => Self::Text {
                        value: inner.as_str().trim_matches('\'').to_string(),
                    },
                    Rule::NUMBER => Self::Number {
                        value: inner.as_str().to_string(),
                    },
                    other => panic!("Expected STRING_LITERAL or NUMBER, not rule {other:?}"),
                }
            }
        }
    }
}

impl Sql for DefaultValue {
    fn as_sql(&self) -> String {
        TEMPLATES
            .render(
                "default_value/template.sql",
                &tera::Context::from_serialize(self).unwrap(),
            )
            .expect("Failed to render default value sql")
            .trim()
            .to_string()
    }
}
