use crate::parser::{types::TEMPLATES, Sql};
use tera::Context;

#[derive(Debug, Clone)]
pub struct Object {
    pub column_names: Vec<String>,
}

impl Object {
    pub fn new(column_names: Vec<String>) -> Self {
        Self { column_names }
    }
}

impl Sql for Object {
    fn as_sql(&self) -> String {
        let mut ctx = Context::new();

        ctx.insert("column_names", &self.column_names);

        TEMPLATES
            .render("primary_key", &ctx)
            .expect("Failed to render primary key sql")
    }
}
