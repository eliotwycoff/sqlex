use crate::parser::{types::TEMPLATES, Rule, Sql};
use pest::iterators::Pair;

#[derive(Debug, Clone)]
pub struct PrimaryKey {
    pub name: Option<String>,
    pub column_names: Vec<String>,
}

impl From<Pair<'_, Rule>> for PrimaryKey {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let mut inner = pair.into_inner();

        match inner.peek().expect("Expected an inner rule").as_rule() {
            Rule::INDEX_NAME => {
                let name = inner
                    .next()
                    .map(|p| p.as_str().trim_matches('`').to_string());
                let columns = inner
                    .map(|col| col.as_str().trim_matches('`').to_string())
                    .collect::<Vec<String>>();

                PrimaryKey::new(name, columns)
            }
            Rule::QUOTED_IDENTIFIER => {
                let columns = inner
                    .map(|col| col.as_str().trim_matches('`').to_string())
                    .collect::<Vec<String>>();

                PrimaryKey::new(None, columns)
            }
            rule => panic!("Expected an INDEX_NAME or a QUOTED_IDENTIFIER, not {rule:?}"),
        }
    }
}

impl PrimaryKey {
    pub fn new(name: Option<String>, column_names: Vec<String>) -> Self {
        Self { name, column_names }
    }
}

impl Sql for PrimaryKey {
    fn as_sql(&self) -> String {
        let mut ctx = tera::Context::new();

        ctx.insert("name", &self.name);
        ctx.insert("column_names", &self.column_names);

        TEMPLATES
            .render("primary_key/template.sql", &ctx)
            .expect("Failed to render primary key sql")
            .trim()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::MySqlParser;
    use pest::Parser;

    #[test]
    fn can_parse_single_primary_key_without_name() {
        let primary_key = PrimaryKey::from(
            MySqlParser::parse(Rule::PRIMARY_KEY, "PRIMARY KEY (`id`),")
                .expect("Invalid input")
                .next()
                .expect("Unable to parse input"),
        );

        assert!(primary_key.name.is_none());
        assert_eq!(primary_key.column_names, vec![String::from("id")]);
    }

    #[test]
    fn can_parse_multiple_primary_key_without_name() {
        let primary_key = PrimaryKey::from(
            MySqlParser::parse(Rule::PRIMARY_KEY, "PRIMARY KEY (`id1`, `id2`, `id3`),")
                .expect("Invalid input")
                .next()
                .expect("Unable to parse input"),
        );

        assert!(primary_key.name.is_none());
        assert_eq!(
            primary_key.column_names,
            vec![
                String::from("id1"),
                String::from("id2"),
                String::from("id3"),
            ]
        );
    }

    #[test]
    fn can_parse_single_primary_key_with_name() {
        let primary_key = PrimaryKey::from(
            MySqlParser::parse(Rule::PRIMARY_KEY, "CONSTRAINT `pk` PRIMARY KEY (`id`),")
                .expect("Invalid input")
                .next()
                .expect("Unable to parse input"),
        );

        assert_eq!(primary_key.name.unwrap().as_str(), "pk");
        assert_eq!(primary_key.column_names, vec![String::from("id")]);
    }

    #[test]
    fn can_parse_multiple_primary_key_with_name() {
        let primary_key = PrimaryKey::from(
            MySqlParser::parse(
                Rule::PRIMARY_KEY,
                "CONSTRAINT `pk` PRIMARY KEY (`id1`, `id2`, `id3`),",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );

        assert_eq!(primary_key.name.unwrap().as_str(), "pk");
        assert_eq!(
            primary_key.column_names,
            vec![
                String::from("id1"),
                String::from("id2"),
                String::from("id3"),
            ]
        )
    }

    #[test]
    fn can_write_primary_key_without_name() {
        let primary_key = PrimaryKey {
            name: None,
            column_names: vec![String::from("id1"), String::from("id2")],
        };

        assert_eq!(primary_key.as_sql().trim(), "PRIMARY KEY (`id1`,`id2`)",);
    }

    #[test]
    fn can_write_primary_key_with_name() {
        let primary_key = PrimaryKey {
            name: Some(String::from("pk")),
            column_names: vec![String::from("id1"), String::from("id2")],
        };

        assert_eq!(
            primary_key.as_sql().trim(),
            "CONSTRAINT `pk` PRIMARY KEY (`id1`,`id2`)",
        );
    }
}
