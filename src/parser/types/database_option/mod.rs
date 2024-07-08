use crate::parser::{types::TEMPLATES, Rule, Sql};
use pest::iterators::Pair;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum DatabaseOption {
    CharacterSet { default: bool, value: String },
    Collate { default: bool, value: String },
    Encryption { default: bool, value: String },
}

impl From<Pair<'_, Rule>> for DatabaseOption {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let default = pair.as_str().contains("DEFAULT");
        let option = pair.into_inner().next().expect("database option");
        let value = option
            .clone()
            .into_inner()
            .next()
            .expect("database option value")
            .as_str()
            .trim_matches('\'')
            .to_string();

        match option.as_rule() {
            Rule::CHARACTER_SET => Self::CharacterSet { default, value },
            Rule::COLLATE => Self::Collate { default, value },
            Rule::ENCRYPTION => Self::Encryption { default, value },
            other => panic!("Expected CHARACTER_SET, COLLATE or ENCRYPTION, not {other:?}"),
        }
    }
}

impl Sql for DatabaseOption {
    fn as_sql(&self) -> String {
        TEMPLATES
            .render(
                "database_option/template.sql",
                &tera::Context::from_serialize(self).unwrap(),
            )
            .expect("Failed to render database option sql")
    }
}

#[cfg(test)]
mod test {
    use std::ops::Not;

    use super::*;
    use crate::parser::MySqlParser;
    use pest::Parser;

    #[test]
    fn can_parse_database_options() {
        let database_options = MySqlParser::parse(
            Rule::DATABASE_OPTIONS,
            "DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT ENCRYPTION='N'",
        )
        .expect("Invalid input")
        .next()
        .expect("Unable to parse input")
        .into_inner()
        .map(|p| DatabaseOption::from(p))
        .collect::<Vec<DatabaseOption>>();

        match &database_options[0] {
            DatabaseOption::CharacterSet { default, value } => {
                assert!(default);
                assert_eq!(value.as_str(), "utf8mb4");
            }
            _ => panic!("Expected character set"),
        };

        match &database_options[1] {
            DatabaseOption::Collate { default, value } => {
                assert!(default.not());
                assert_eq!(value.as_str(), "utf8mb4_general_ci");
            }
            _ => panic!("Expected collate"),
        }

        match &database_options[2] {
            DatabaseOption::Encryption { default, value } => {
                assert!(default);
                assert_eq!(value.as_str(), "N");
            }
            _ => panic!("Expected encryption"),
        }

        assert_eq!(database_options.len(), 3);
    }

    #[test]
    fn can_write_database_options() {
        assert_eq!(
            vec![
                DatabaseOption::CharacterSet {
                    default: true,
                    value: String::from("utf8mb4")
                },
                DatabaseOption::Collate {
                    default: false,
                    value: String::from("utf8mb4_general_ci")
                },
                DatabaseOption::Encryption {
                    default: true,
                    value: String::from("N")
                }
            ]
            .into_iter()
            .map(|opt| opt.as_sql().trim().to_string())
            .collect::<Vec<String>>()
            .join(" ")
            .as_str(),
            "DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT ENCRYPTION='N'"
        );
    }
}
