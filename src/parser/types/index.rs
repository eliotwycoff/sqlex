use crate::parser::Sql;

#[derive(Debug, Clone)]
pub struct Object {
    pub name: String,
    pub columns: Vec<String>,
    pub unique: bool,
}

impl Object {
    pub fn new(name: String, columns: Vec<String>, unique: bool) -> Self {
        Self {
            name,
            columns,
            unique,
        }
    }
}

impl Sql for Object {
    fn as_sql(&self) -> String {
        todo!()
    }
}
