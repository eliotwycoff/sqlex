use crate::parser::{parser::Rule, Sql};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum DatabaseOption {
    CharacterSet(String),
    Collate(String),
    Encryption(String),
}

impl Sql for DatabaseOption {
    fn as_sql(&self) -> String {
        match self {
            DatabaseOption::CharacterSet(value) => format!("CHARACTER_SET {value}"),
            DatabaseOption::Collate(value) => format!("COLLATE {value}"),
            DatabaseOption::Encryption(value) => format!("ENCRYPTION {value}"),
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

impl Sql for Database {
    fn as_sql(&self) -> String {
        let options_sql = if self.options.is_empty() {
            String::from("")
        } else {
            format!(
                "DEFAULT {}",
                self.options
                    .iter()
                    .map(|opt| opt.as_sql())
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        };

        let tables_sql = self
            .tables
            .values()
            .map(|value| value.as_sql())
            .collect::<Vec<String>>()
            .join("\n\n");

        format!(
            r#"
            --
            -- Current Database: `{}`
            --

            CREATE DATABASE IF NOT EXISTS {} {};

            USE `{}`;
        
            {}

            "#,
            self.name, self.name, options_sql, self.name, tables_sql,
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct Table {
    // Table settings
    pub name: String,
    pub columns: Vec<Column>,
    pub primary_key: Option<String>,
    pub indexes: Vec<Index>,

    // Table options
    pub engine: Option<String>,
    pub auto_increment: Option<String>,
    pub charset: Option<String>,
    pub collate: Option<String>,

    // Row operations
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
            auto_increment: None,
            charset: None,
            collate: None,
            inserts: Vec::new(),
            updates: Vec::new(),
            deletes: Vec::new(),
        }
    }
}

impl Sql for Table {
    fn as_sql(&self) -> String {
        // TODO: Finish this.
        let engine = String::from("");
        let auto_increment = String::from("");
        let charset = String::from("");
        let collate = String::from("");

        // TODO: Finish this
        format!(
            r#"
            --
            -- Table structure for table `{}` 
            --

            DROP TABLE IF EXISTS `{}`;
            CREATE TABLE `{}` (
              -- TODO: FINISH THIS
            ) {} {} {} {}
            "#,
            self.name, self.name, self.name, engine, auto_increment, charset, collate,
        )
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

impl Sql for Column {
    fn as_sql(&self) -> String {
        todo!()
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

impl Sql for Index {
    fn as_sql(&self) -> String {
        todo!()
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

impl Sql for Insert {
    fn as_sql(&self) -> String {
        todo!()
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

impl Sql for Update {
    fn as_sql(&self) -> String {
        todo!()
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

impl Sql for Delete {
    fn as_sql(&self) -> String {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Set {
    pub variable: String,
    pub value: String,
}

impl Sql for Set {
    fn as_sql(&self) -> String {
        todo!()
    }
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

impl Sql for DataType {
    fn as_sql(&self) -> String {
        todo!()
    }
}
