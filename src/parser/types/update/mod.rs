use crate::parser::{Rule, Sql};
use pest::iterators::Pair;
use std::collections::HashMap;

use super::Where;

#[derive(Debug, Clone)]
pub struct Update {
    pub table_name: String,
    pub set_clauses: HashMap<String, String>,
    pub where_clauses: Vec<Where>,
}

impl Update {
    pub fn new(table_name: String, set_clauses: HashMap<String, String>) -> Self {
        Self {
            table_name,
            set_clauses,
            where_clauses: vec![],
        }
    }
}

impl From<Pair<'_, Rule>> for Update {
    fn from(pair: Pair<'_, Rule>) -> Self {
        println!("Update from pair: {:#?}", pair);
        let mut inner = pair.into_inner();
        let name = inner
            .next()
            .map(|p| p.as_str().trim_matches('`').to_string())
            .expect("Expected an index name");
        println!("inner: {:?}", name);
        let columns: Vec<String> = inner
            .map(|col| col.as_str().trim_matches('`').to_string())
            .collect();

        Update::new(name, HashMap::new())
    }
}

impl Sql for Update {
    fn as_sql(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{MySqlParser, Rule};
    use pest::Parser;

    use super::*;

    #[test]
    fn test_can_parse_update() {
        let sql = "UPDATE `users` SET `name` = 'John' WHERE `id` = 1;";
        let mut parsed = MySqlParser::parse(Rule::UPDATE_STATEMENT, sql).unwrap();
        let update_stmt = parsed.next().unwrap();
        println!("parsed: {:?}", update_stmt);
        let update = Update::from(update_stmt);
        println!("update: {:?}", update);
        // let update = Update::new(
        //     "users".to_string(),
        //     HashMap::from([("name".to_string(), "John".to_string())]),
        // );
        // assert_eq!(update.as_sql(), sql);
    }
}
