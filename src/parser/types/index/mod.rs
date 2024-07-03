use crate::parser::Sql;

#[derive(Debug, Clone)]
pub struct Index {
    pub name: String,
    pub columns: Vec<String>,
    pub unique: bool,
}

impl Index {
    pub fn new(name: String, columns: Vec<String>, unique: bool) -> Self {
        Self {
            name,
            columns,
            unique,
        }
    }
}

impl Sql for Index {
    fn as_sql(&self) -> String {
        todo!()
    }
}
