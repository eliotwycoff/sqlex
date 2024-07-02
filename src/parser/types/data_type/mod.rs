use crate::parser::Sql;

#[derive(Debug, Clone)]
pub enum Object {
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

impl Default for Object {
    fn default() -> Self {
        Object::Varchar(None)
    }
}

impl Sql for Object {
    fn as_sql(&self) -> String {
        todo!()
    }
}
