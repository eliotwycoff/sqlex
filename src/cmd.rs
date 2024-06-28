use std::{collections::HashMap, path::Path};

use clap::Parser;

use crate::{simple_parse, sqlparse::to_json, types::Database};

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

    #[arg(short, long)]
    pub query: Option<String>,
}

pub fn exec() {
    let args = Args::parse();

    let sqlfile_path = Path::new(&args.sql_file);
    if !sqlfile_path.exists() {
        eprintln!("File {} does not exist", sqlfile_path.display());
        std::process::exit(1);
    }

    if let Some(query) = args.query {
        let res = simple_parse(sqlfile_path).expect("unable to load input file");
        // let input = to_json(res.clone());
        let result = find_pass_columns(&res, &query);
        println!("{}", serde_json::to_string(&result).unwrap());
    } else {
        let res = simple_parse(sqlfile_path).expect("unable to load input file");
        let input = to_json(res.clone());
        println!("{}", input.to_string());
    }
}

#[derive(Debug, serde::Serialize)]
struct Result {
    db_name: String,
    table_name: String,
}

fn find_pass_columns(databases: &Vec<Database>, query_str: &str) -> Vec<Result> {
    let mut result = Vec::new();

    for database in databases {
        let db_name = database.db_name.clone();
        for table in &database.tables {
            for column in &table.columns {
                if column
                    .name
                    .to_lowercase()
                    .contains(&query_str.to_lowercase())
                {
                    result.push(Result {
                        db_name: db_name.clone(),
                        table_name: table.name.clone(),
                    });
                    break; // Move to the next table after finding the first matching column
                }
            }
        }
    }

    result
}
