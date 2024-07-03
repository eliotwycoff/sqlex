use crate::parser::Sql;

#[derive(Debug, Clone)]
pub struct Delete {
    pub table_name: String,
    pub where_clause: Option<String>,
}

impl Delete {
    pub fn new(table_name: String, where_clause: Option<String>) -> Self {
        Self {
            table_name,
            where_clause,
        }
    }
}

impl Sql for Delete {
    fn as_sql(&self) -> String {
        todo!()
    }
}
