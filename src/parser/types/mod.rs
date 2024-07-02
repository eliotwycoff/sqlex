use lazy_static::lazy_static;
use tera::Tera;

mod column;
mod data_type;
mod database;
mod database_option;
mod delete;
mod index;
mod insert;
mod set;
mod table;
mod update;

pub use column::Object as Column;
pub use data_type::Object as DataType;
pub use database::Object as Database;
pub use database_option::Object as DatabaseOption;
pub use delete::Object as Delete;
pub use index::Object as Index;
pub use insert::Object as Insert;
pub use set::Object as Set;
pub use table::Object as Table;
pub use update::Object as Update;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        match Tera::new("src/**/*.sql") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error: {e}");

                ::std::process::exit(1);
            }
        }
    };
}
