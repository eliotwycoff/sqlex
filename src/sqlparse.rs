use regex::Regex;
use std::{collections::HashMap, path::Path}; // 1.1.8

use sql_parse::{
    parse_statements, CreateDefinition, CreateTable, ParseOptions, QualifiedName, SQLDialect,
    Statement,
};

use crate::{
    types::{Column, ColumnType, Database, Table},
    ExtractResult,
};

pub fn simple_parse(code_path: &Path) -> ExtractResult<Vec<Database>> {
    let options = ParseOptions::new()
        .dialect(SQLDialect::MariaDB)
        .arguments(sql_parse::SQLArguments::QuestionMark)
        .warn_unquoted_identifiers(true);

    let mut issues = Vec::new();

    let sql_dump = std::fs::read_to_string(code_path).expect("unable to read sql dump");

    // Regex to capture the `USE` statement and the database name
    let db_regex = Regex::new(r"USE `([^`]+)`;").unwrap();

    // Split by `USE` while retaining the delimiters
    let mut databases = Vec::new();
    let mut current_db_name = String::new();
    let mut current_db_sql = String::new();

    for line in sql_dump.lines() {
        if let Some(captures) = db_regex.captures(line) {
            // Process the previous database if any
            if !current_db_name.is_empty() {
                let ast = parse_statements(&current_db_sql, &mut issues, &options);
                let mut tables = Vec::new();
                for node in ast.iter() {
                    match node {
                        Statement::CreateTable(create_table) => {
                            let tbl = parse_create_table(create_table);
                            tables.push(tbl.clone());
                        }
                        _ => {}
                    }
                }
                databases.push(Database {
                    db_name: current_db_name.clone(),
                    tables,
                });

                // Clear the SQL statements for the new database
                current_db_sql.clear();
            }
            // Capture the new database name
            current_db_name = captures.get(1).unwrap().as_str().to_string();
        } else {
            // Append the line to the current database's SQL statements
            current_db_sql.push_str(line);
            current_db_sql.push('\n');
        }
    }

    // Process the last database if any
    if !current_db_name.is_empty() {
        let ast = parse_statements(&current_db_sql, &mut issues, &options);
        let mut tables = Vec::new();
        for node in ast.iter() {
            match node {
                Statement::CreateTable(create_table) => {
                    let tbl = parse_create_table(create_table);
                    tables.push(tbl.clone());
                }
                _ => {}
            }
        }
        databases.push(Database {
            db_name: current_db_name,
            tables,
        });
    }

    Ok(databases)
}

pub fn to_json(databases: Vec<Database>) -> serde_json::Value {
    let mut json: HashMap<String, Database> = HashMap::new();
    for database in databases {
        json.insert(database.db_name.clone(), database);
    }
    serde_json::to_value(json).unwrap()
}

fn parse_create_table(create_table: &CreateTable) -> Table {
    let table_name =
        extract_table_name(&create_table.identifier).expect("unable to parse table name");
    let table_columns = extract_table_columns(create_table).expect("unable to parse table columns");

    Table {
        name: table_name,
        columns: table_columns.clone(),
        constraints: None,
    }
}

fn extract_table_name(identifier: &QualifiedName) -> ExtractResult<String> {
    let identifier = identifier.identifier.clone();
    Ok(identifier.value.to_string())
}

fn extract_table_columns(create_table: &CreateTable) -> ExtractResult<Vec<Column>> {
    let columns = create_table
        .create_definitions
        .iter()
        .filter_map(|definition| extract_column_definition(definition).transpose()) // Use transpose to convert Option<Result<T, E>> to Result<Option<T>, E>
        .collect::<Result<Vec<_>, _>>()?; // Now correctly collecting into Result<Vec<_>, _>
    Ok(columns)
}

fn extract_column_definition(definition: &CreateDefinition) -> ExtractResult<Option<Column>> {
    match definition {
        CreateDefinition::ColumnDefinition {
            identifier,
            data_type,
        } => {
            let type_ = ColumnType::from(data_type.type_.clone());
            let column = Column {
                name: identifier.value.to_string(),
                type_,
            };
            Ok(Some(column.clone()))
        }
        CreateDefinition::ConstraintDefinition { .. } => Ok(None),
    }
}
