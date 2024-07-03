use crate::parser::{types::TEMPLATES, Sql};
use tera::Context;

#[derive(Debug, Clone)]
pub struct PrimaryKey {
    pub name: Option<String>,
    pub column_names: Vec<String>,
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
