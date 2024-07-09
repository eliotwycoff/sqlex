use crate::parser::{
    types::{DatabaseOption, TEMPLATES},
    Rule, Sql,
};
use pest::iterators::Pair;
use std::collections::HashMap;
use tera::Context;

#[derive(Debug, Clone, Default)]
pub struct Database {
    pub name: String,
    pub options: Vec<DatabaseOption>,
}

impl Database {
    pub fn new(name: String) -> Self {
        Self {
            name,
            options: Vec::new(),
        }
    }
}

impl From<Pair<'_, Rule>> for Database {
    fn from(pair: Pair<Rule>) -> Self {
        let mut inner_pair = pair.into_inner();
        let mut db = Self::new(
            inner_pair
                .next()
                .unwrap()
                .as_str()
                .trim_matches('`')
                .to_string(),
        );

        if let Some(option) = inner_pair.next() {
            let mut inner_options_pair = option.into_inner();
            let mut options = Vec::new();

            while let Some(option) = inner_options_pair.next() {
                if let Some(pair) = option.into_inner().next() {
                    options.push(DatabaseOption::from(pair))
                }
            }

            db.options = options;
        }

        db
    }
}

impl Sql for Database {
    fn as_sql(&self) -> String {
        let mut ctx = Context::new();

        ctx.insert("name", self.name.as_str());
        ctx.insert(
            "options",
            &self
                .options
                .iter()
                .map(|opt| opt.as_sql())
                .collect::<Vec<String>>(),
        );

        TEMPLATES
            .render("database/template.sql", &ctx)
            .expect("Failed to render database sql")
    }
}
