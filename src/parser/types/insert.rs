use crate::parser::Sql;

#[derive(Debug, Clone)]
pub struct Object {
    pub columns: Vec<String>,
    pub values: Vec<String>,
}

impl Object {
    pub fn new(columns: Vec<String>, values: Vec<String>) -> Self {
        Self { columns, values }
    }
}

impl Sql for Object {
    fn as_sql(&self) -> String {
        todo!()
    }
}
