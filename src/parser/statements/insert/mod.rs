use crate::parser::{statements::TEMPLATES, types::InsertValues, Rule, Sql};
use pest::iterators::Pair;

#[derive(Debug, Clone)]
pub struct Insert {
    pub table_name: String,
    pub column_names: Vec<String>,
    pub values: Vec<InsertValues>,
}
