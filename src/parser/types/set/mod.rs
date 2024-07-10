use pest::iterators::{Pair, Pairs};

use crate::parser::{parse_utils::trim_str, Rule, Sql};

#[derive(Debug, Clone)]
pub struct KVPair {
    pub key: String,
    pub value: String,
}

impl KVPair {
    pub fn new(key: String, value: String) -> Self {
        KVPair { key, value }
    }
}

#[derive(Debug, Clone)]
pub struct Set {
    pub kv_pairs: Vec<KVPair>,
}

impl Sql for Set {
    fn as_sql(&self) -> String {
        todo!()
    }
}

impl From<Pair<'_, Rule>> for Set {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let inner = pair.into_inner();
        let kv_pairs: Vec<KVPair> = inner
            .into_iter()
            .map(|p| {
                let mut part = p.into_inner();
                let name = trim_str(part.next().unwrap());
                let value = trim_str(part.next().unwrap());
                KVPair::new(name, value)
            })
            .collect();
        Set { kv_pairs }
    }
}

impl From<Pairs<'_, Rule>> for Set {
    fn from(pairs: Pairs<'_, Rule>) -> Self {
        let set: Vec<Set> = pairs.into_iter().map(Set::from).collect();
        set[0].clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    use crate::parser::{MySqlParser, Rule};

    use super::*;

    #[test]
    fn test_parses_set_into_single_kv_pair() {
        let sql = "SET @a = 1";
        let parsed = MySqlParser::parse(Rule::SET_STATEMENT, sql);
        assert!(parsed.is_ok());
        let set = Set::from(parsed.unwrap());
        let kvs = set.kv_pairs;
        assert_eq!(kvs.len(), 1);
        assert_eq!(kvs[0].key, "a");
        assert_eq!(kvs[0].value, "1");
    }

    #[test]
    fn test_parses_set_into_k_v_pairs() {
        let sql = "SET @a = 1, @b = 2, name = 'John'";
        let parsed = MySqlParser::parse(Rule::SET_STATEMENT, sql);
        assert!(parsed.is_ok());
        let set = Set::from(parsed.unwrap());
        let kvs = set.kv_pairs;
        assert_eq!(kvs.len(), 3);
        assert_eq!(kvs[0].key, "a");
        assert_eq!(kvs[0].value, "1");
        assert_eq!(kvs[1].key, "b");
        assert_eq!(kvs[1].value, "2");
        assert_eq!(kvs[2].key, "name");
        assert_eq!(kvs[2].value, "John");
    }

    #[test]
    fn test_errors_with_invalid_set_stmt() {
        let sql = "SET @a > 1";
        let parsed = MySqlParser::parse(Rule::SET_STATEMENT, sql);
        assert!(parsed.is_err());
    }
}
