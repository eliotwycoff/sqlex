use crate::parser::{types::DataType, Rule, Sql};
use pest::iterators::Pair;

#[derive(Debug, Clone, Default)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
    pub nullable: bool,
    pub default: Option<String>,
    pub auto_increment: bool,
    pub unique: bool,
}

impl Column {
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

impl From<Pair<'_, Rule>> for Column {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let mut inner = pair.into_inner();
        let name = inner.next().unwrap().as_str().trim_matches('`').to_string();
        let data_type = DataType::from(inner.next().unwrap());
        let mut column = Column::new(name, data_type);

        for constraint in inner {
            match constraint.as_str() {
                "NOT NULL" => column.nullable = false,
                "NULL" => column.nullable = true,
                s if s.starts_with("DEFAULT") => {
                    column.default = Some(
                        s.strip_prefix("DEFAULT ")
                            .unwrap()
                            .trim_matches('\'')
                            .to_string(),
                    )
                }
                "AUTO_INCREMENT" => column.auto_increment = true,
                "PRIMARY KEY" => {}
                _ => {}
            }
        }

        column
    }
}

impl Sql for Column {
    fn as_sql(&self) -> String {
        todo!()
    }
}
