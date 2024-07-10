use crate::parser::{Rule, Sql};
use pest::iterators::Pair;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone)]
pub struct InsertValues {}
