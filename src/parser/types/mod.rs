use lazy_static::lazy_static;
use tera::Tera;

mod column;
mod data_type;
mod database;
mod database_option;
mod delete;
mod index;
mod insert;
mod primary_key;
mod set;
mod table;
mod update;

pub use column::Column;
pub use data_type::DataType;
pub use database::Database;
pub use database_option::DatabaseOption;
pub use delete::Delete;
pub use index::Index;
pub use insert::Insert;
pub use primary_key::PrimaryKey;
pub use set::Set;
pub use table::Table;
pub use update::Update;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        match Tera::new("src/parser/types/**/*.sql") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error: {e}");

                ::std::process::exit(1);
            }
        }
    };
}
