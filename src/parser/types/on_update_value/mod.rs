use crate::parser::{types::TEMPLATES, Rule, Sql};
use pest::iterators::Pair;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum OnUpdateValue {
    Cascade,
    CurrentTimestamp { value: Option<u32> },
}

impl From<Pair<'_, Rule>> for OnUpdateValue {
    fn from(pair: Pair<'_, Rule>) -> Self {
        match pair
            .as_str()
            .to_ascii_uppercase()
            .split_ascii_whitespace()
            .next()
            .expect("keyword")
            .split("(")
            .next()
            .expect("keyword before (")
        {
            "CASCADE" => Self::Cascade,
            "CURRENT_TIMESTAMP" => Self::CurrentTimestamp {
                value: pair
                    .into_inner()
                    .next()
                    .map(|p| p.as_str().parse::<u32>().unwrap()),
            },
            other => panic!("Expected CASCADE or CURRENT_TIMESTAMP, not {other:?}"),
        }
    }
}

impl Sql for OnUpdateValue {
    fn as_sql(&self) -> String {
        TEMPLATES
            .render(
                "on_update_value/template.sql",
                &tera::Context::from_serialize(self).unwrap(),
            )
            .expect("Failed to render on update value sql")
            .trim()
            .to_string()
    }
}
