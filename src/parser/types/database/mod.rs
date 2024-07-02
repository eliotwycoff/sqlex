use crate::parser::{
    types::{DatabaseOption, Table, TEMPLATES},
    Sql,
};
use std::collections::HashMap;
use tera::Context;

#[derive(Debug, Clone, Default)]
pub struct Object {
    pub name: String,
    pub tables: HashMap<String, Table>,
    pub options: Vec<DatabaseOption>,
}

impl Object {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tables: HashMap::new(),
            options: Vec::new(),
        }
    }
}

impl Sql for Object {
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
        ctx.insert(
            "tables",
            &self
                .tables
                .values()
                .map(|table| table.as_sql())
                .collect::<Vec<String>>(),
        );

        TEMPLATES
            .render("database", &ctx)
            .expect("Failed to render database sql")
    }
}
