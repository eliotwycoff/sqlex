use crate::parser::Sql;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Update {
    pub table_name: String,
    pub set_clauses: HashMap<String, String>,
}

impl Update {
    pub fn new(table_name: String, set_clauses: HashMap<String, String>) -> Self {
        Self {
            table_name,
            set_clauses,
        }
    }
}

impl Sql for Update {
    fn as_sql(&self) -> String {
        todo!()
    }
}
