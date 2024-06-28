use std::{collections::HashMap, path::Path};

use sql_parse::{
    parse_statements, CreateDefinition, CreateTable, ParseOptions, QualifiedName, SQLDialect,
    Statement,
};

use crate::{
    types::{Column, ColumnType, Table},
    ExtractResult,
};

pub fn simple_parse(code_path: &Path) -> ExtractResult<Vec<Table>> {
    let options = ParseOptions::new()
        .dialect(SQLDialect::MariaDB)
        .arguments(sql_parse::SQLArguments::QuestionMark)
        .warn_unquoted_identifiers(true);

    let mut issues = Vec::new();

    let sql_dump = std::fs::read_to_string(code_path).expect("unable to read sql dump");

    let ast = parse_statements(&sql_dump, &mut issues, &options);

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

    Ok(tables)
}

pub fn to_json(tables: Vec<Table>) -> serde_json::Value {
    let mut json: HashMap<String, Table> = HashMap::new();
    for table in tables {
        json.insert(table.name.clone(), table);
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
    // if let CreateDefinition::ColumnDefinition {
    //     identifier,
    //     data_type,
    // } = definition
    // {
    //     let type_ = ColumnType::from(data_type.type_.clone());
    //     let column = Column {
    //         name: identifier.value.to_string(),
    //         type_,
    //     };
    //     Ok(column.clone())
    // } else {
    //     println!("Error occurred: {:?}", definition);
    //     Err(Box::new(std::io::Error::new(
    //         std::io::ErrorKind::InvalidInput,
    //         "unable to parse column definition",
    //     )))
    // }
}
