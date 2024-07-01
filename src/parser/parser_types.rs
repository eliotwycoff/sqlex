use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Database {
    pub name: String,
    pub tables: HashMap<String, Table>,
    pub set_variables: HashMap<String, String>,
}

impl Database {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tables: HashMap::new(),
            set_variables: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Table {
    pub database_name: String,
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
    pub fn new(database_name: String, name: String) -> Self {
        Self {
            database_name,
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
