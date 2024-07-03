use crate::parser::{types::TEMPLATES, Rule, Sql};
use pest::iterators::Pair;

#[derive(Debug, Clone)]
pub struct ForeignKey {
    pub name: Option<String>,
    pub local_column_names: Vec<String>,
    pub foreign_column_names: Vec<String>,
    pub foreign_table_name: String,
}

impl From<Pair<'_, Rule>> for ForeignKey {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let mut inner = pair.into_inner();

        match inner.peek().expect("Expected an inner rule").as_rule() {
            Rule::INDEX_NAME => {
                let name = inner
                    .next()
                    .map(|p| p.as_str().trim_matches('`').to_string());

                // TODO: Finish this.
                todo!()
            }
            Rule::QUOTED_IDENTIFIER => {
                // TODO: Finish this.
                todo!()
            }
            rule => panic!("Expected an INDEX_NAME or a QUOTED_IDENTIFIER, not {rule:?}"),
        }
    }
}

impl ForeignKey {
    pub fn new(
        name: Option<String>,
        local_column_names: Vec<String>,
        foreign_column_names: Vec<String>,
        foreign_table_name: String,
    ) -> Self {
        Self {
            name,
            local_column_names,
            foreign_column_names,
            foreign_table_name,
        }
    }
}

impl Sql for ForeignKey {
    fn as_sql(&self) -> String {
        let mut ctx = tera::Context::new();

        ctx.insert("name", &self.name);
        ctx.insert("local_column_names", &self.local_column_names);
        ctx.insert("foreign_column_names", &self.foreign_column_names);
        ctx.insert("foreign_table_name", &self.foreign_table_name);

        TEMPLATES
            .render("foreign_key/template.sql", &ctx)
            .expect("Failed to render foreign key sql")
    }
}
