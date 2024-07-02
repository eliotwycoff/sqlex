use crate::parser::{types::DataType, Sql};

#[derive(Debug, Clone, Default)]
pub struct Object {
    pub name: String,
    pub data_type: DataType,
    pub nullable: bool,
    pub default: Option<String>,
    pub auto_increment: bool,
    pub unique: bool,
}

impl Object {
    pub fn new(name: String, data_type: DataType) -> Self {
        Self {
            name,
            data_type,
            nullable: true,
            default: None,
            auto_increment: false,
            unique: false,
        }
    }
}

impl Sql for Object {
    fn as_sql(&self) -> String {
        todo!()
    }
}
