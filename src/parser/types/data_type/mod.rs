use crate::parser::{Rule, Sql};
use pest::iterators::Pair;

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

impl From<Pair<'_, Rule>> for DataType {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let type_name = pair
            .as_str()
            .split('(')
            .next()
            .unwrap()
            .trim()
            .to_uppercase();
        let mut inner = pair.into_inner();

        match type_name.as_str() {
            "TINYINT" | "SMALLINT" | "MEDIUMINT" | "INT" | "INTEGER" | "BIGINT" | "BIT" => {
                let size = inner.next().map(|p| p.as_str().parse::<u32>().unwrap());
                match type_name.as_str() {
                    "TINYINT" => DataType::TinyInt(size),
                    "SMALLINT" => DataType::SmallInt(size),
                    "MEDIUMINT" => DataType::MediumInt(size),
                    "INT" | "INTEGER" => DataType::Int(size),
                    "BIGINT" => DataType::BigInt(size),
                    "BIT" => DataType::Bit(size),
                    _ => unreachable!(),
                }
            }
            "DECIMAL" | "NUMERIC" | "FLOAT" | "DOUBLE" => {
                let precision = inner.next().map(|p| p.as_str().parse::<u32>().unwrap());
                let scale = inner.next().map(|p| p.as_str().parse::<u32>().unwrap());
                match type_name.as_str() {
                    "DECIMAL" | "NUMERIC" => {
                        DataType::Decimal(precision.and_then(|p| scale.map(|s| (p, s))))
                    }
                    "FLOAT" => DataType::Float(precision.and_then(|p| scale.map(|s| (p, s)))),
                    "DOUBLE" => DataType::Double(precision.and_then(|p| scale.map(|s| (p, s)))),
                    _ => unreachable!(),
                }
            }
            "DATE" => DataType::Date,
            "DATETIME" | "TIMESTAMP" | "TIME" | "YEAR" => {
                let size = inner.next().map(|p| p.as_str().parse::<u32>().unwrap());
                match type_name.as_str() {
                    "DATETIME" => DataType::DateTime(size),
                    "TIMESTAMP" => DataType::Timestamp(size),
                    "TIME" => DataType::Time(size),
                    "YEAR" => DataType::Year(size),
                    _ => unreachable!(),
                }
            }
            "CHAR" | "VARCHAR" | "BINARY" | "VARBINARY" => {
                let size = inner
                    .next()
                    .map(|p| p.as_str().parse::<u32>().unwrap())
                    .unwrap();
                match type_name.as_str() {
                    "CHAR" => DataType::Char(Some(size)),
                    "VARCHAR" => DataType::Varchar(Some(size)),
                    "BINARY" => DataType::Binary(Some(size)),
                    "VARBINARY" => DataType::Varbinary(Some(size)),
                    _ => unreachable!(),
                }
            }
            "TINYBLOB" => DataType::TinyBlob,
            "BLOB" => DataType::Blob,
            "MEDIUMBLOB" => DataType::MediumBlob,
            "LONGBLOB" => DataType::LongBlob,
            "TINYTEXT" => DataType::TinyText,
            "TEXT" => DataType::Text,
            "MEDIUMTEXT" => DataType::MediumText,
            "LONGTEXT" => DataType::LongText,
            "ENUM" => {
                let values: Vec<String> = inner
                    .map(|p| p.as_str().trim_matches('\'').to_string())
                    .collect();
                DataType::Enum(values)
            }
            "SET" => {
                let values: Vec<String> = inner
                    .map(|p| p.as_str().trim_matches('\'').to_string())
                    .collect();
                DataType::Set(values)
            }
            "GEOMETRY" => DataType::Geometry,
            "POINT" => DataType::Point,
            "LINESTRING" => DataType::LineString,
            "POLYGON" => DataType::Polygon,
            "MULTIPOINT" => DataType::MultiPoint,
            "MULTILINESTRING" => DataType::MultiLineString,
            "MULTIPOLYGON" => DataType::MultiPolygon,
            "GEOMETRYCOLLECTION" => DataType::GeometryCollection,
            "JSON" => DataType::JSON,
            _ => unimplemented!("Data type {} not implemented", type_name),
        }
    }
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
