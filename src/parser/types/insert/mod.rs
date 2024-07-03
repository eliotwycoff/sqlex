use crate::parser::Sql;

#[derive(Debug, Clone)]
pub struct Insert {
    pub columns: Vec<String>,
    pub values: Vec<String>,
}

impl Insert {
    pub fn new(columns: Vec<String>, values: Vec<String>) -> Self {
        Self { columns, values }
    }
}

impl Sql for Insert {
    fn as_sql(&self) -> String {
        todo!()
    }
}
