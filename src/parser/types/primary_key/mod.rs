use crate::parser::{types::TEMPLATES, Rule, Sql};
use pest::iterators::Pair;
use tera::Context;

#[derive(Debug, Clone)]
pub struct PrimaryKey {
    pub name: Option<String>,
    pub column_names: Vec<String>,
}

impl From<Pair<'_, Rule>> for PrimaryKey {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let mut inner = pair.into_inner();

        match inner.peek().expect("Expected an inner rule").as_rule() {
            Rule::INDEX_NAME => {
                let name = inner
                    .next()
                    .map(|p| p.as_str().trim_matches('`').to_string());
                let columns = inner
                    .map(|col| col.as_str().trim_matches('`').to_string())
                    .collect::<Vec<String>>();

                PrimaryKey::new(name, columns)
            }
            Rule::QUOTED_IDENTIFIER => {
                let columns = inner
                    .map(|col| col.as_str().trim_matches('`').to_string())
                    .collect::<Vec<String>>();

                PrimaryKey::new(None, columns)
            }
            rule => panic!("Expected an INDEX_NAME or a QUOTED_IDENTIFIER, not {rule:?}"),
        }
    }
}

impl PrimaryKey {
    pub fn new(name: Option<String>, column_names: Vec<String>) -> Self {
        Self { name, column_names }
    }
}

impl Sql for PrimaryKey {
    fn as_sql(&self) -> String {
        let mut ctx = Context::new();

        ctx.insert("name", &self.name);
        ctx.insert("column_names", &self.column_names);

        TEMPLATES
            .render("primary_key", &ctx)
            .expect("Failed to render primary key sql")
    }
}
