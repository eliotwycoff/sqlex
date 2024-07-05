use crate::parser::{
    types::{DataType, TEMPLATES},
    Rule, Sql,
};
use pest::iterators::Pair;

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
    pub nullable: bool,
    pub default: Option<String>,
    pub auto_increment: bool,
    pub comment: Option<String>,
}

impl Column {
    pub fn new(name: String, data_type: DataType) -> Self {
        Self {
            name,
            data_type,
            nullable: true,
            default: None,
            auto_increment: false,
            comment: None,
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
                "PRIMARY KEY" => todo!("support for PRIMARY KEY"),
                s if s.starts_with("COMMENT") => {
                    column.comment = Some(
                        s.strip_prefix("COMMENT ")
                            .unwrap()
                            .trim_matches('\'')
                            .to_string(),
                    )
                }
                other => todo!("support for {}", other),
            }
        }

        column
    }
}

impl Sql for Column {
    fn as_sql(&self) -> String {
        let mut ctx = tera::Context::new();

        ctx.insert("name", &self.name);
        ctx.insert("data_type", &self.data_type.as_sql());
        ctx.insert("nullable", &self.nullable);
        ctx.insert("default", &self.default);
        ctx.insert("auto_increment", &self.auto_increment);
        ctx.insert("comment", &self.comment);

        TEMPLATES
            .render("column/template.sql", &ctx)
            .expect("Failed to render column sql")
    }
}
