use crate::parser::Sql;

#[derive(Debug, Clone)]
pub struct Object {
    pub variable: String,
    pub value: String,
}

impl Sql for Object {
    fn as_sql(&self) -> String {
        todo!()
    }
}
