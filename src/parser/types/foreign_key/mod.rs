use crate::parser::{types::TEMPLATES, Rule, Sql};
use pest::iterators::Pair;

#[derive(Debug, Clone)]
pub struct ForeignKey {
    pub name: Option<String>,
    pub local_column_names: Vec<String>,
    pub foreign_column_names: Vec<String>,
    pub foreign_table_name: String,
}

impl From<Pair<'_, Rule>> for ForeignKey {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let mut inner = pair.into_inner();
        let name = match inner.peek().expect("Expected an inner rule").as_rule() {
            Rule::INDEX_NAME => inner
                .next()
                .map(|p| p.as_str().trim_matches('`').to_string()),
            _ => None,
        };
        let (local_column_names, foreign_table_name, foreign_column_names) = inner.fold(
            (Vec::new(), String::new(), Vec::new()),
            |(mut local, mut table, mut foreign), pair| {
                match pair.as_rule() {
                    Rule::QUOTED_IDENTIFIER => {
                        table
                            .is_empty()
                            .then(|| &mut local)
                            .unwrap_or_else(|| &mut foreign)
                            .push(pair.as_str().trim_matches('`').to_string());
                    }
                    Rule::TABLE_NAME => {
                        table = pair.as_str().trim_matches('`').to_string();
                    }
                    rule => {
                        panic!("Expected QUOTED_IDENTIFIER or TABLE_NAME, not {rule:?}")
                    }
                };

                (local, table, foreign)
            },
        );

        Self::new(
            name,
            local_column_names,
            foreign_column_names,
            foreign_table_name,
        )
    }
}

impl ForeignKey {
    pub fn new(
        name: Option<String>,
        local_column_names: Vec<String>,
        foreign_column_names: Vec<String>,
        foreign_table_name: String,
    ) -> Self {
        Self {
            name,
            local_column_names,
            foreign_column_names,
            foreign_table_name,
        }
    }
}

impl Sql for ForeignKey {
    fn as_sql(&self) -> String {
        let mut ctx = tera::Context::new();

        ctx.insert("name", &self.name);
        ctx.insert("local_column_names", &self.local_column_names);
        ctx.insert("foreign_column_names", &self.foreign_column_names);
        ctx.insert("foreign_table_name", &self.foreign_table_name);

        TEMPLATES
            .render("foreign_key/template.sql", &ctx)
            .expect("Failed to render foreign key sql")
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
    fn can_parse_single_foreign_key_without_name() {
        let foreign_key = ForeignKey::from(
            MySqlParser::parse(
                Rule::FOREIGN_KEY,
                "FOREIGN KEY (`column_id`) REFERENCES `column` (`id`),",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );

        assert!(foreign_key.name.is_none());
        assert_eq!(
            foreign_key.local_column_names,
            vec![String::from("column_id")]
        );
        assert_eq!(foreign_key.foreign_column_names, vec![String::from("id")]);
    }

    #[test]
    fn can_parse_multiple_foreign_key_without_name() {
        let foreign_key = ForeignKey::from(
            MySqlParser::parse(
                Rule::FOREIGN_KEY,
                "FOREIGN KEY (`column_id`, `column_name`) REFERENCES `column` (`id`, `name`),",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );

        assert!(foreign_key.name.is_none());
        assert_eq!(
            foreign_key.local_column_names,
            vec![String::from("column_id"), String::from("column_name")]
        );
        assert_eq!(
            foreign_key.foreign_column_names,
            vec![String::from("id"), String::from("name")]
        );
    }

    #[test]
    fn can_parse_single_foreign_key_with_name() {
        let foreign_key = ForeignKey::from(
            MySqlParser::parse(
                Rule::FOREIGN_KEY,
                "CONSTRAINT `fk_column_id` FOREIGN KEY (`column_id`) REFERENCES `column` (`id`),",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );

        assert_eq!(foreign_key.name.unwrap().as_str(), "fk_column_id");
        assert_eq!(
            foreign_key.local_column_names,
            vec![String::from("column_id")]
        );
        assert_eq!(foreign_key.foreign_column_names, vec![String::from("id")]);
    }

    #[test]
    fn can_parse_multiple_foreign_key_with_name() {
        let foreign_key = ForeignKey::from(
            MySqlParser::parse(
                Rule::FOREIGN_KEY,
                "CONSTRAINT `fk_column` FOREIGN KEY (`column_id`, `column_name`) REFERENCES `column` (`id`, `name`),",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );

        assert_eq!(foreign_key.name.unwrap().as_str(), "fk_column");
        assert_eq!(
            foreign_key.local_column_names,
            vec![String::from("column_id"), String::from("column_name")]
        );
        assert_eq!(
            foreign_key.foreign_column_names,
            vec![String::from("id"), String::from("name")]
        );
    }

    #[test]
    fn can_write_foreign_key_without_name() {
        let foreign_key = ForeignKey {
            name: None,
            local_column_names: vec![String::from("column_id"), String::from("column_name")],
            foreign_column_names: vec![String::from("id"), String::from("name")],
            foreign_table_name: String::from("column"),
        };

        assert_eq!(
            foreign_key.as_sql().trim(),
            "FOREIGN KEY (`column_id`,`column_name`) REFERENCES `column` (`id`,`name`)",
        );
    }

    #[test]
    fn can_write_foreign_key_with_name() {
        let foreign_key = ForeignKey {
            name: Some(String::from("fk_column")),
            local_column_names: vec![String::from("column_id"), String::from("column_name")],
            foreign_column_names: vec![String::from("id"), String::from("name")],
            foreign_table_name: String::from("column"),
        };

        assert_eq!(
            foreign_key.as_sql().trim(),
            "CONSTRAINT `fk_column` FOREIGN KEY (`column_id`,`column_name`) REFERENCES `column` (`id`,`name`)",
        );
    }
}
