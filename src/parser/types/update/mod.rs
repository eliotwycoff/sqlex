use crate::parser::Sql;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Object {
    pub table_name: String,
    pub set_clauses: HashMap<String, String>,
}

impl Object {
    pub fn new(table_name: String, set_clauses: HashMap<String, String>) -> Self {
        Self {
            table_name,
            set_clauses,
        }
    }
}

impl Sql for Object {
    fn as_sql(&self) -> String {
        todo!()
    }
}
