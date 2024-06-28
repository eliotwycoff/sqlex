use std::path::Path;

use sql_parse::{parse_statements, ParseOptions, SQLDialect, Statement};
use tree_sitter::{Query, QueryCursor};

pub const QUERY: &str = include_str!("query.scm");

pub fn simple_parse(code_path: &Path) {
    let options = ParseOptions::new()
        .dialect(SQLDialect::MariaDB)
        .arguments(sql_parse::SQLArguments::QuestionMark)
        .warn_unquoted_identifiers(true);

    let mut issues = Vec::new();

    let sql_dump = std::fs::read_to_string(code_path).expect("unable to read sql dump");

    let ast = parse_statements(&sql_dump, &mut issues, &options);

    for node in ast.iter() {
        match node {
            Statement::CreateTable { table, .. } => {
                println!("table: {:#?}", table);
            }
            _ => {}
        }
    }
}

pub fn parse_sql(code_path: &Path) {
    let lang = tree_sitter_sql::language();
    let mut parser = tree_sitter::Parser::new();
    let _ = parser.set_language(lang);

    let sql_dump = std::fs::read_to_string(code_path).expect("unable to read sql dump");

    let tree = parser
        .parse(&sql_dump, None)
        .expect("Error parsing SQL dump");

    let query = Query::new(lang, QUERY).expect("Error creating query");

    let mut query_cursor = QueryCursor::new();
    let root_node = tree.root_node();

    let matches = query_cursor.matches(&query, root_node, sql_dump.as_bytes());

    for mat in matches {
        let mut table_name = "";
        let mut columns = Vec::new();
        let mut column_name = "";

        for capture in mat.captures.iter() {
            let node = capture.node;
            let text = node
                .utf8_text(sql_dump.as_bytes())
                .expect("Error getting text");

            println!("text: {}", text);

            match query.capture_names()[capture.index as usize].as_str() {
                "table_name" => table_name = text,
                "column_name" => {
                    column_name = text;
                }
                "column_type" => {
                    columns.push((column_name, text.to_string()));
                }
                other => {
                    println!("other: {}", other);
                }
            }
        }

        println!("Table: {}", table_name);
        for (name, typ) in columns {
            println!("  Column: {} Type: {}", name, typ);
        }
    }
}
