use crate::parser::{Rule, Sql};
use pest::iterators::Pair;

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

impl From<Pair<'_, Rule>> for Index {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let mut inner = pair.into_inner();
        let index_type = inner.next().unwrap().as_str();
        let name = inner
            .next()
            .map(|p| p.as_str().trim_matches('`').to_string())
            .unwrap_or_else(|| format!("index_{}", uuid::Uuid::new_v4()));
        let columns: Vec<String> = inner
            .map(|col| col.as_str().trim_matches('`').to_string())
            .collect();
        let unique = index_type.contains("UNIQUE") || index_type == "PRIMARY KEY";

        Index::new(name, columns, unique)
    }
}

impl Sql for Index {
    fn as_sql(&self) -> String {
        todo!()
    }
}
