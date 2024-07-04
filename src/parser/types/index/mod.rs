use crate::parser::{types::TEMPLATES, Rule, Sql};
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
        let unique = inner.next().unwrap().as_str().contains("UNIQUE");
        let name = inner
            .next()
            .map(|p| p.as_str().trim_matches('`').to_string())
            .expect("Expected an index name");
        let columns: Vec<String> = inner
            .map(|col| col.as_str().trim_matches('`').to_string())
            .collect();

        Index::new(name, columns, unique)
    }
}

impl Sql for Index {
    fn as_sql(&self) -> String {
        let mut ctx = tera::Context::new();

        ctx.insert("name", &self.name);
        ctx.insert("column_names", &self.columns);
        ctx.insert("unique", &self.unique);

        TEMPLATES
            .render("index/template.sql", &ctx)
            .expect("Failed to render index sql")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::MySqlParser;
    use pest::Parser;
    use std::ops::Not;

    #[test]
    fn can_parse_single_non_unique_index() {
        let index = Index::from(
            MySqlParser::parse(
                Rule::INDEX_DEFINITION,
                "KEY `recipient_id` (`recipient_id`),",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );

        assert_eq!(index.name.as_str(), "recipient_id");
        assert_eq!(index.columns, vec![String::from("recipient_id")]);
        assert!(index.unique.not());
    }

    #[test]
    fn can_parse_multiple_non_unique_index() {
        let index = Index::from(
            MySqlParser::parse(
                Rule::INDEX_DEFINITION,
                "KEY `recipient` (`recipient_id`, `recipient_name`),",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );

        assert_eq!(index.name.as_str(), "recipient");
        assert_eq!(
            index.columns,
            vec![String::from("recipient_id"), String::from("recipient_name")]
        );
        assert!(index.unique.not());
    }

    #[test]
    fn can_parse_single_unique_index() {
        let index = Index::from(
            MySqlParser::parse(
                Rule::INDEX_DEFINITION,
                "UNIQUE KEY `recipient_id` (`recipient_id`),",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );

        assert_eq!(index.name.as_str(), "recipient_id");
        assert_eq!(index.columns, vec![String::from("recipient_id")]);
        assert!(index.unique);
    }

    #[test]
    fn can_parse_multiple_unique_index() {
        let index = Index::from(
            MySqlParser::parse(
                Rule::INDEX_DEFINITION,
                "UNIQUE KEY `recipient` (`recipient_id`, `recipient_name`),",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );

        assert_eq!(index.name.as_str(), "recipient");
        assert_eq!(
            index.columns,
            vec![String::from("recipient_id"), String::from("recipient_name")]
        );
        assert!(index.unique);
    }

    #[test]
    fn can_write_non_unique_index() {
        let index = Index {
            name: String::from("recipient"),
            columns: vec![String::from("recipient_id"), String::from("recipient_name")],
            unique: false,
        };

        assert_eq!(
            index.as_sql().trim(),
            "KEY `recipient` (`recipient_id`,`recipient_name`)",
        );
    }

    #[test]
    fn can_write_unique_index() {
        let index = Index {
            name: String::from("recipient"),
            columns: vec![String::from("recipient_id"), String::from("recipient_name")],
            unique: true,
        };

        assert_eq!(
            index.as_sql().trim(),
            "UNIQUE KEY `recipient` (`recipient_id`,`recipient_name`)",
        );
    }
}
