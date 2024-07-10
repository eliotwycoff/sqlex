use lazy_static::lazy_static;
use tera::Tera;

mod create_database;
mod create_table;
mod drop_table;
mod use_database;

pub use create_database::CreateDatabase;
pub use create_table::CreateTable;
pub use drop_table::DropTable;
pub use use_database::UseDatabase;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        match Tera::new("src/parser/statements/**/*.sql") {
            Ok(s) => s,
            Err(e) => {
                println!("Parsing error: {e}");

                ::std::process::exit(1);
            }
        }
    };
}
