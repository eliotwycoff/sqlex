use crate::parser::Sql;

#[derive(Debug, Clone)]
pub struct Object {
    pub table_name: String,
    pub where_clause: Option<String>,
}

impl Object {
    pub fn new(table_name: String, where_clause: Option<String>) -> Self {
        Self {
            table_name,
            where_clause,
        }
    }
}

impl Sql for Object {
    fn as_sql(&self) -> String {
        todo!()
    }
}
