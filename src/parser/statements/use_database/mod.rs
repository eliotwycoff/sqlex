use crate::parser::{statements::TEMPLATES, Rule, Sql};
use pest::iterators::Pair;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct UseDatabase {
    pub name: String,
}

impl From<Pair<'_, Rule>> for UseDatabase {
    fn from(pair: Pair<'_, Rule>) -> Self {
        Self {
            name: pair
                .into_inner()
                .next()
                .expect("QUOTED_IDENTIFIER")
                .as_str()
                .trim_matches('`')
                .to_string(),
        }
    }
}

impl Sql for UseDatabase {
    fn as_sql(&self) -> String {
        TEMPLATES
            .render(
                "use_database/template.sql",
                &tera::Context::from_serialize(self).unwrap(),
            )
            .expect("Failed to render use database sql")
            .trim()
            .to_string()
    }
}
