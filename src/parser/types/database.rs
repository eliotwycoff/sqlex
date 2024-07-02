use crate::parser::{
    types::{DatabaseOption, Table},
    Sql,
};
use std::collections::HashMap;

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
        let options_sql = if self.options.is_empty() {
            String::from("")
        } else {
            format!(
                "DEFAULT {}",
                self.options
                    .iter()
                    .map(|opt| opt.as_sql())
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        };
        let tables_sql = self
            .tables
            .values()
            .map(|value| value.as_sql())
            .collect::<Vec<String>>()
            .join("\n\n");

        format!(
            r#"
            --
            -- Current Object: `{}`
            --

            CREATE Object IF NOT EXISTS {} {};

            USE `{}`;
        
            {}

            "#,
            self.name, self.name, options_sql, self.name, tables_sql,
        )
    }
}
