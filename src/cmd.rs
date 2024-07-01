#[allow(unused)]
use rayon::prelude::*;
use sql_parse::Statement::{self, InsertReplace};
use sql_parse::{parse_statements, Expression, ParseOptions, SQLDialect};
use std::path::Path;

use clap::Parser;
use sql_script_parser::sql_script_parser;

use crate::parser::parser::MyParser;
use crate::rules::get_struct_by_name;
use crate::ExtractResult;
use crate::{settings::parse_masking_config, simple_parse, sqlparse::to_json, types::Database};

#[allow(unused)]
static DEFAULT_JSON_FILTER: &str = r#"to_entries | map({table: .key, columns: .value.columns | map(select(.name | test("pass"; "i")))}) | map(select(.columns | length > 0))"#;

#[derive(Parser)]
#[command(about = format!("
Extract SQL from a text file

Usage:
--sql-file <sql_file>

--query <query>
"))]
pub struct Args {
    #[arg(short, long)]
    pub sql_file: String,

    #[arg(short, long)]
    pub query: Option<String>,

    #[command(subcommand)]
    pub cmd: Option<Commands>,
}

#[derive(Parser)]
pub enum Commands {
    #[command(about = "Mask PII from a SQL file")]
    MaskPII(MaskPIIArgs),
}

#[derive(Parser)]
pub struct MaskPIIArgs {
    #[arg(short, long)]
    pub sql_file: String,

    #[arg(short, long)]
    masking_config: Option<String>,
}

pub fn exec() -> ExtractResult<Vec<String>> {
    let args = Args::parse();

    match args.cmd {
        Some(Commands::MaskPII(args)) => {
            run_mask_pii_action(&args);
        }
        _ => {
            run_default_action(&args);
        }
    }
    Ok(vec![])
}

/// Mask PII from a SQL file
///  
/// 1. Parse the SQL file and print the JSON representation of the SQL.
/// 2. If the `--mask-pii` flag is provided, mask the PII in the SQL file.
///
/// Returns a list of statements to be executed.
fn run_mask_pii_action(args: &MaskPIIArgs) -> ExtractResult<Vec<Statement>> {
    let sqlfile_path = Path::new(&args.sql_file);
    if !sqlfile_path.exists() {
        eprintln!("File {} does not exist", sqlfile_path.display());
        std::process::exit(1);
    }

    let file_bytes = std::fs::read(sqlfile_path).expect("unable to read file");
    let file_str = std::str::from_utf8(&file_bytes).expect("Invalid UTF-8 sequence");

    let masking_config = args.masking_config.clone().unwrap_or_default();
    let config = parse_masking_config(&masking_config).expect("unable to load masking config");
    // let parser = sql_script_parser(&file_bytes).map(|x| x.statement);
    let mut my_parser = MyParser::new();
    my_parser = my_parser.parse(file_str).unwrap();
    println!("my parser: {:?}", my_parser);

    // let mut issues = Vec::new();
    // let options = ParseOptions::new()
    //     .dialect(SQLDialect::MariaDB)
    //     .arguments(sql_parse::SQLArguments::QuestionMark)
    //     .warn_unquoted_identifiers(true);

    // parser.into_iter().for_each(|x| {
    //     // take a string and parse it into a list of statements
    //     // where each is a single SQL operation
    //     let sql_str = std::str::from_utf8(x).expect("Invalid UTF-8 sequence");
    // let ast = parse_statements(sql_str, &mut issues, &options);
    //     // since we're only dealing with insert replace statements
    //     // which are a single operation per statement
    //     // we can assume there's only one. If there are more statements
    //     // we are just going to ignore them as they aren't an insert or replace statement
    //     if ast.len() == 1 {
    //         match &ast[0] {
    //             InsertReplace(sql_parse::InsertReplace {
    //                 columns, values, ..
    //             }) => {
    //                 // In an insert replace statement, we are dealing with values and their columns that match
    //                 // the value.
    //                 // To create a masking application, we'll walk through each of the columns matched with their
    //                 // values. If the value is a column that we are interested in given by the filter configuration
    //                 // then we'll match up the appropriate rule and apply it to the value, replacing the original
    //                 // value with the masked value.
    //                 let values = values.as_ref().unwrap();
    //                 let cols =
    //                     columns
    //                         .into_iter()
    //                         .zip(values.clone().1.into_iter())
    //                         .map(|(col, val)| {
    //                             if config.filter_column(&col.value.to_string()) {
    //                                 let masking_fn = get_struct_by_name(&col.value.to_string());
    //                                 let new_val = masking_fn.fake();
    //                                 let new_stmt = Expression::StringLiteral(new_val.to_string());
    //                                 (col, new_stmt)
    //                             } else {
    //                                 (col, val)
    //                             }
    //                         });
    //                 let cols =
    //                     columns
    //                         .into_iter()
    //                                 .zip(values.clone().1.into_iter())
    //                                 .filter(|(x, _val)| config.filter_column(&x.value.to_string()))
    //                                 .collect::<Vec<(
    //                                     &sql_parse::Identifier<'_>,
    //                                     Vec<sql_parse::Expression<'_>>,
    //                                 )>>();
    //                 let mut new_values = Vec::new();
    //                 for (col, _val) in cols {
    //                     let fn_ = get_struct_by_name(&col.to_string());
    //                     // let rule = MaskingRule(fn_);
    //                     // let val = rule.fake();
    //                     let val = fn_.fake();
    //                     new_values.push(val);
    //                 }
    //             }
    //             _ => {}
    //         }
    //     } else {
    //         // ast.iter().for_each(|x| {
    //         //     println!("{:?}", x);
    //         // });
    //     }
    // });
    Ok(vec![])
}

/// Run a single replacement
fn create_new_statement_for_masking<'a>(
    config: &crate::settings::MaskingConfig,
    statements: Vec<Statement<'a>>,
) -> ExtractResult<Vec<Statement<'a>>> {
    statements.into_iter().for_each(|x| match x {
        InsertReplace(sql_parse::InsertReplace {
            columns, values, ..
        }) => {
            println!("{:?}", columns);
            println!("{:?}", values);
        }
        _ => {}
    });
    Ok(vec![])
}

/// Default action.
///
/// 1. Parse the SQL file and print the JSON representation of the SQL.
/// 2. If the `--query` flag is provided, print the columns that contain the query string.
/// 3. If the `--mask-pii` flag is provided, mask the PII in the SQL file.
fn run_default_action(args: &Args) -> ExtractResult<Vec<String>> {
    let sqlfile_path = Path::new(&args.sql_file);
    if !sqlfile_path.exists() {
        eprintln!("File {} does not exist", sqlfile_path.display());
        std::process::exit(1);
    }
    let mut vals: Vec<String> = Vec::new();
    if let Some(query) = args.query.as_ref() {
        let res = simple_parse(sqlfile_path).expect("unable to load input file");
        // let input = to_json(res.clone());
        let result = find_pass_columns(&res, &query);
        println!("{}", serde_json::to_string(&result).unwrap());
    } else {
        let res = simple_parse(sqlfile_path).expect("unable to load input file");
        let input = to_json(res.clone());
        println!("{}", input.to_string());
        vals.push(input.to_string());
    }
    Ok(vals)
}

#[derive(Debug, serde::Serialize)]
struct Result {
    db_name: String,
    table_name: String,
    column_name: String,
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
                        column_name: column.name.clone(),
                    });
                    break; // Move to the next table after finding the first matching column
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use std::{io::Write, path::PathBuf};
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_replace_single_statement() {
        let temp_dir = tempfile::tempdir().unwrap();
        let test_sql_file = create_temp_sql_with_insert(&temp_dir);
        let test_config = create_test_masking_config(&temp_dir);

        let parsed_config =
            parse_masking_config(&test_config.as_os_str().to_str().unwrap()).unwrap();

        let sql_single_insert = r#"INSERT INTO users (id, name, email, password) VALUES (1, 'John Doe', 'john.doe@example.com', 'password');"#;

        let mut issues = Vec::new();
        let options = ParseOptions::new()
            .dialect(SQLDialect::MariaDB)
            .arguments(sql_parse::SQLArguments::QuestionMark)
            .warn_unquoted_identifiers(true);

        let ast = parse_statements(&sql_single_insert, &mut issues, &options);

        let _ = create_new_statement_for_masking(&parsed_config, ast);

        // let res = run_mask_pii_action(&MaskPIIArgs {
        //     sql_file: "./tests/schema_dump.sql".to_string(),
        //     masking_config: Some("./tests/more.yaml".to_string()),
        // });
        // assert!(res.is_ok());
    }

    #[test]
    fn test_can_extract_sql_from_file() {
        let args = MaskPIIArgs {
            sql_file: "./tests/schema_dump.sql".to_string(),
            masking_config: Some("./tests/more.yaml".to_string()),
        };
        let res = run_mask_pii_action(&args);
        println!("{:?}", res);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(res.len() == 1);
    }

    fn create_test_masking_config(temp_dir: &TempDir) -> PathBuf {
        let temp_file_in_path = temp_dir.path().join("test.yaml");
        let test_config = r#"
columns:
    - account
patterns:
    - name: email
      regex: ^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$

rules:
    email: contact::email()"#;
        let mut file = std::fs::File::create(temp_file_in_path.clone()).unwrap();
        file.write_all(test_config.as_bytes()).unwrap();
        file.flush().unwrap();
        file.sync_data().unwrap();
        temp_file_in_path
    }

    fn create_temp_sql_with_insert(temp_dir: &TempDir) -> PathBuf {
        let temp_file_in_path = temp_dir.path().join("test.sql");
        let sql_single_insert = r#"USE `users`;\nINSERT INTO users (id, name, email, password) VALUES (1, 'John Doe', 'john.doe@example.com', 'password');"#;
        let mut file = std::fs::File::create(temp_file_in_path.clone()).unwrap();
        file.write_all(sql_single_insert.as_bytes()).unwrap();
        file.flush().unwrap();
        file.sync_data().unwrap();
        temp_file_in_path
    }
}
