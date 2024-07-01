pub mod parser;
pub mod parser_types;

pub trait Sql {
    fn as_sql(&self) -> String;
}
