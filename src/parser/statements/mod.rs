use lazy_static::lazy_static;
use tera::Tera;

mod create_table;

pub use create_table::CreateTable;

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
