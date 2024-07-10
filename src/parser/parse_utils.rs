use pest::iterators::Pair;

use super::Rule;

pub fn trim_str(s: Pair<'_, Rule>) -> String {
    s.as_str().trim_matches('`').trim_matches('\'').to_string()
}
