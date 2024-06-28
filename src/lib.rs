pub mod cmd;
pub mod sqlparse;
pub mod types;

pub type ExtractResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

pub use sqlparse::simple_parse;
