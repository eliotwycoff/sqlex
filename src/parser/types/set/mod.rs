use crate::parser::Sql;

#[derive(Debug, Clone)]
pub struct Set {
    pub variable: String,
    pub value: String,
}

impl Sql for Set {
    fn as_sql(&self) -> String {
        todo!()
    }
}
