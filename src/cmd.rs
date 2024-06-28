use std::path::Path;

use clap::Parser;

use crate::{simple_parse, sqlparse::to_json};

#[allow(unused)]
static DEFAULT_JSON_FILTER: &str = r#"to_entries | map({table: .key, columns: .value.columns | map(select(.name | test("pass"; "i")))}) | map(select(.columns | length > 0))"#;

#[derive(Parser)]
#[command(about = format!("
Extract SQL from a text file

Usage:
--sql-file <sql_file>

Hint: extract columns with the word 'pass' using `jq`:

sqlex --sql-file ./schema_dump.sql | jq '{DEFAULT_JSON_FILTER}'
"))]
pub struct Args {
    #[arg(short, long)]
    pub sql_file: String,
}

pub fn exec() {
    let args = Args::parse();

    let sqlfile_path = Path::new(&args.sql_file);
    if !sqlfile_path.exists() {
        eprintln!("File {} does not exist", sqlfile_path.display());
        std::process::exit(1);
    }
    let res = simple_parse(sqlfile_path).expect("unable to load input file");
    let input = to_json(res.clone());

    println!("{}", input.to_string());
}
