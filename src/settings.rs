use std::collections::HashMap;

use config::{Config, ConfigError, Environment, File};

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::rules::{self, get_struct_by_name};

// lazy_static! {
//     static ref FN_NAMES_TO_MODULE: HashMap<String, Box<dyn Fn(String) -> String>> = {
//         "internet" => Box::new(fakeit::internet),
//     };
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct MaskingRegex {
    pub name: Option<String>,
    pub regex: String,
}

#[derive(Debug)]
pub struct MaskingRule(Box<dyn rules::FromStrFaking>);

impl MaskingRule {
    pub fn inner(&self) -> &dyn rules::FromStrFaking {
        &*self.0
    }
}

impl From<&str> for MaskingRule {
    fn from(value: &str) -> Self {
        let mut parts = value.split("::");
        let _domain = parts.next().unwrap();
        let fn_name = parts.next().unwrap();
        let fn_ = get_struct_by_name(fn_name);
        MaskingRule(fn_)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaskingConfig {
    pub columns: Vec<String>,
    pub patterns: Vec<MaskingRegex>,
    #[serde(skip)]
    pub regexes: Vec<Regex>,
    #[serde(skip)]
    pub rules: HashMap<String, MaskingRule>,
}

impl MaskingConfig {
    pub fn filter_column(&self, column: &str) -> bool {
        let lowercased_columns: Vec<String> =
            self.columns.iter().map(|c| c.to_lowercase()).collect();
        let lowercased_column = column.to_lowercase();

        if lowercased_columns.contains(&lowercased_column) {
            return true;
        }

        for regex in &self.build_regexes() {
            if regex.is_match(&column) {
                return true;
            }
        }
        false
    }

    fn build_regexes(&self) -> Vec<Regex> {
        self.patterns
            .iter()
            .map(|regex| Regex::new(&regex.regex).unwrap())
            .collect()
    }
}

pub fn parse_masking_config(path: &str) -> std::result::Result<MaskingConfig, ConfigError> {
    let s = Config::builder()
        // Start off by merging in the "default" configuration file
        .add_source(File::with_name(path))
        .add_source(File::with_name("./config/local").required(false))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(Environment::with_prefix("SQLEX_"))
        // You may also programmatically change settings
        .build()?;

    s.try_deserialize()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_extract_config() {
        let config = parse_masking_config("./tests/more.yaml").unwrap();
        assert_eq!(config.patterns.len(), 1);
        assert_eq!(
            config.patterns[0].regex,
            "^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$"
        );
    }

    #[test]
    fn test_filtering_columns() {
        let config = parse_masking_config("./tests/more.yaml");
        let cfg = config.unwrap();
        assert_eq!(cfg.filter_column("email"), false);
        assert_eq!(cfg.filter_column("account"), true);
        assert_eq!(cfg.filter_column("password"), true);
        assert_eq!(cfg.filter_column("age"), false);
    }
}
