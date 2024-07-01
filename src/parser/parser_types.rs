use std::{collections::HashMap, fmt::Display};

use super::format_trait::SqlFormat;
use crate::parser::parser::Rule;

#[derive(Debug, Clone)]
pub enum DatabaseOption {
    CharacterSet(String),
    Collate(String),
    Encryption(String),
}

impl Display for DatabaseOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseOption::CharacterSet(value) => write!(f, "character_set_database = {}", value),
            DatabaseOption::Collate(value) => write!(f, "collate = {}", value),
            DatabaseOption::Encryption(value) => write!(f, "encryption = {}", value),
        }
    }
}

impl DatabaseOption {
    pub fn from_pair(pair: pest::iterators::Pair<Rule>) -> Option<DatabaseOption> {
        if let Some(inner_option) = pair.into_inner().next() {
            let key = match inner_option.as_rule() {
                Rule::CHARACTER_SET => DatabaseOption::CharacterSet,
                Rule::COLLATE => DatabaseOption::Collate,
                Rule::ENCRYPTION => DatabaseOption::Encryption,
                _ => return None,
            };
            let value = match inner_option.into_inner().next() {
                Some(value) => value.as_str().trim_matches('`').to_string(),
                None => String::new(),
            };
            Some(key(value))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Database {
    pub name: String,
    pub tables: HashMap<String, Table>,
    pub options: Vec<DatabaseOption>,
}

impl Database {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tables: HashMap::new(),
            options: Vec::new(),
        }
    }
}

impl SqlFormat for Database {
    fn format_str(&self) -> String {
        format!(r#"CREATE DATABASE IF NOT EXISTS {}"#, self.name)
    }
    fn format(&self) -> String {
        "".to_string()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub primary_key: Option<String>,
    pub indexes: Vec<Index>,
    pub engine: Option<String>,
    pub charset: Option<String>,
    pub collation: Option<String>,
    pub inserts: Vec<Insert>,
    pub updates: Vec<Update>,
    pub deletes: Vec<Delete>,
}

impl Table {
    pub fn new(name: String) -> Self {
        Self {
            name,
            columns: Vec::new(),
            primary_key: None,
            indexes: Vec::new(),
            engine: None,
            charset: None,
            collation: None,
            inserts: Vec::new(),
            updates: Vec::new(),
            deletes: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
    pub nullable: bool,
    pub default: Option<String>,
    pub auto_increment: bool,
    pub unique: bool,
}

impl Column {
    pub fn new(name: String, data_type: DataType) -> Self {
        Self {
            name,
            data_type,
            nullable: true,
            default: None,
            auto_increment: false,
            unique: false,
        }
    }
}
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

#[derive(Debug, Clone)]
pub struct Insert {
    pub columns: Vec<String>,
    pub values: Vec<String>,
}

impl Insert {
    pub fn new(columns: Vec<String>, values: Vec<String>) -> Self {
        Self { columns, values }
    }
}

#[derive(Debug, Clone)]
pub struct Update {
    pub table_name: String,
    pub set_clauses: HashMap<String, String>,
}

impl Update {
    pub fn new(table_name: String, set_clauses: HashMap<String, String>) -> Self {
        Self {
            table_name,
            set_clauses,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Delete {
    pub table_name: String,
    pub where_clause: Option<String>,
}

impl Delete {
    pub fn new(table_name: String, where_clause: Option<String>) -> Self {
        Self {
            table_name,
            where_clause,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Set {
    pub variable: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub enum DataType {
    TinyInt(Option<u32>),
    SmallInt(Option<u32>),
    MediumInt(Option<u32>),
    Int(Option<u32>),
    BigInt(Option<u32>),
    Decimal(Option<(u32, u32)>),
    Float(Option<(u32, u32)>),
    Double(Option<(u32, u32)>),
    Bit(Option<u32>),
    Boolean,
    Date,
    DateTime(Option<u32>),
    Timestamp(Option<u32>),
    Time(Option<u32>),
    Year(Option<u32>),
    Char(Option<u32>),
    Varchar(Option<u32>),
    Binary(Option<u32>),
    Varbinary(Option<u32>),
    TinyBlob,
    Blob,
    MediumBlob,
    LongBlob,
    TinyText,
    Text,
    MediumText,
    LongText,
    Enum(Vec<String>),
    Set(Vec<String>),
    Geometry,
    Point,
    LineString,
    Polygon,
    MultiPoint,
    MultiLineString,
    MultiPolygon,
    GeometryCollection,
    JSON,
}

impl Default for DataType {
    fn default() -> Self {
        DataType::Varchar(None)
    }
}
