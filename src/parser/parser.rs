use crate::ExtractResult;

use super::parser_types::{Column, DataType, Database, Delete, Index, Insert, Set, Table, Update};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/sql.pest"]
struct MySQLDumpParser;

#[derive(Debug)]
pub struct MyParser {
    pub databases: Vec<Database>,
    pub current_database: Option<Database>,
}

impl MyParser {
    pub fn new() -> Self {
        Self {
            databases: vec![],
            current_database: None,
        }
    }
    pub fn parse(mut self, input: &str) -> ExtractResult<Self> {
        let parsed_databases = parse_mysqldump(input)?;
        self.databases.extend(parsed_databases);
        Ok(self)
    }

    pub fn get_databases(&self) -> &Vec<Database> {
        &self.databases
    }
}

pub fn parse_mysqldump(input: &str) -> Result<Vec<Database>, pest::error::Error<Rule>> {
    let mut parse_result = MySQLDumpParser::parse(Rule::mysqldump, input).expect("invalid input");
    let mysqldump = parse_result.next().expect("invalid input");

    let mut databases = Vec::new();
    let mut current_database: Option<Database> = None;

    for pair in mysqldump.into_inner() {
        match pair.as_rule() {
            Rule::sql_statement => {
                for inner_pair in pair.into_inner() {
                    dbg!(inner_pair.as_rule());
                    match inner_pair.as_rule() {
                        Rule::create_database => {
                            let name = inner_pair
                                .into_inner()
                                .next()
                                .expect("unable to unwrap create_database name")
                                .as_str()
                                .trim_matches('`')
                                .to_string();
                            if let Some(db) = current_database.take() {
                                if db.name != name {
                                    databases.push(db);
                                }
                            }
                            current_database = Some(Database::new(name));
                        }
                        Rule::use_database => {
                            let name = inner_pair
                                .into_inner()
                                .next()
                                .expect("unable to unwrap use_database name")
                                .as_str()
                                .trim_matches('`')
                                .to_string();
                            current_database = Some(Database::new(name.to_string()));
                        }
                        Rule::create_table => {
                            if let Some(ref mut db) = current_database {
                                let table = parse_create_table(inner_pair);
                                db.tables.insert(table.name.clone(), table);
                            }
                        }
                        Rule::alter_table => {
                            if let Some(ref mut db) = current_database {
                                parse_alter_table(inner_pair, db);
                            }
                        }
                        Rule::drop_table => {
                            if let Some(ref mut db) = current_database {
                                let table_name = inner_pair
                                    .clone() // Clone the pair here
                                    .into_inner()
                                    .last()
                                    .expect("unable to extract table name")
                                    .as_str()
                                    .trim_matches('`')
                                    .to_string();
                                db.tables.remove(&table_name);
                            }
                        }
                        Rule::insert_statement => {
                            if let Some(ref mut db) = current_database {
                                let mut inner = inner_pair.into_inner();
                                let table_name =
                                    inner.next().unwrap().as_str().trim_matches('`').to_string();
                                if let Some(table) = db.tables.get_mut(&table_name) {
                                    table.inserts.push(parse_insert_statement(inner));
                                }
                            }
                        }
                        Rule::update_statement => {
                            println!("Found update statement: {:#?}", current_database);
                            if let Some(ref mut db) = current_database {
                                let update = parse_update_statement(inner_pair.into_inner());
                                if let Some(table) = db.tables.get_mut(&update.table_name) {
                                    table.updates.push(update);
                                }
                            }
                        }
                        // Rule::delete_statement => {
                        //     let delete = parse_delete_statement(statement);
                        //     if let Some(table) = db.tables.get_mut(&delete.table_name) {
                        //         table.deletes.push(delete);
                        //     }
                        // }
                        // Rule::set_statement => {
                        //     let set = parse_set_statement(statement);
                        //     db.set_variables.insert(set.variable, set.value);
                        // }
                        // ... existing code ...
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    if let Some(ref db) = current_database {
        databases.push(db.clone());
    }

    Ok(databases)
}

fn parse_create_table(pair: pest::iterators::Pair<Rule>) -> Table {
    let mut inner = pair.into_inner();
    let table_name = inner
        .next()
        .expect("unable to extract table name")
        .as_str()
        .trim_matches('`')
        .to_string();
    let mut table = Table::new(table_name);

    for element in inner {
        match element.as_rule() {
            Rule::column_definition => {
                let column = parse_column_definition(element);
                if column.name == *table.primary_key.as_ref().unwrap_or(&String::new()) {
                    table.primary_key = Some(column.name.clone());
                }
                table.columns.push(column);
            }
            Rule::index_definition => {
                let index = parse_index_definition(element);
                if index.name == "PRIMARY" {
                    table.primary_key = Some(index.columns.first().unwrap().clone());
                } else {
                    table.indexes.push(index);
                }
            }
            _ => {}
        }
    }

    table
}

fn parse_insert_statement(mut pairs: pest::iterators::Pairs<Rule>) -> Insert {
    let column_pairs = pairs.next().expect("invalid insert statement").into_inner();
    let value_pairs = pairs.next().expect("invalid insert statement").into_inner();

    let columns: Vec<String> = column_pairs
        .into_iter()
        .map(|col| col.as_str().trim_matches('`').to_string())
        .collect();

    let values: Vec<String> = value_pairs
        .into_iter()
        .map(|value_list| {
            value_list
                .into_inner()
                .map(|value| value.as_str().trim_matches('\'').to_string())
                .collect::<Vec<String>>()
        })
        .flatten()
        .collect();

    Insert::new(columns, values)
}

fn parse_update_statement(mut pairs: pest::iterators::Pairs<Rule>) -> Update {
    println!("parse update statement: {:#?}", pairs);
    let table_name = pairs
        .next()
        .expect("invalid update statement")
        .as_str()
        .trim_matches('`')
        .to_string();
    let set_statements = pairs.next().expect("invalid update statement").into_inner();
    Update::new(table_name)
}

fn parse_alter_table(pair: pest::iterators::Pair<Rule>, db: &mut Database) {
    let mut inner = pair.into_inner();
    let table_name = inner.next().unwrap().as_str().trim_matches('`').to_string();

    if let Some(table) = db.tables.get_mut(&table_name) {
        for alter_spec in inner {
            match alter_spec.as_rule() {
                Rule::alter_specification => {
                    let mut spec_inner = alter_spec.into_inner();
                    let action = spec_inner.next().unwrap().as_str();
                    match action {
                        "ADD" => {
                            if spec_inner.peek().unwrap().as_rule() == Rule::column_definition {
                                let column = parse_column_definition(spec_inner.next().unwrap());
                                table.columns.push(column);
                            } else {
                                let index = parse_index_definition(spec_inner.next().unwrap());
                                table.indexes.push(index);
                            }
                        }
                        "MODIFY" => {
                            let column = parse_column_definition(spec_inner.next().unwrap());
                            if let Some(existing_column) =
                                table.columns.iter_mut().find(|c| c.name == column.name)
                            {
                                *existing_column = column;
                            }
                        }
                        "DROP" => {
                            let drop_type = spec_inner.next().unwrap().as_str();
                            if drop_type == "COLUMN" {
                                let column_name = spec_inner
                                    .next()
                                    .unwrap()
                                    .as_str()
                                    .trim_matches('`')
                                    .to_string();
                                table.columns.retain(|c| c.name != column_name);
                            } else if drop_type == "INDEX" {
                                let index_name = spec_inner
                                    .next()
                                    .unwrap()
                                    .as_str()
                                    .trim_matches('`')
                                    .to_string();
                                table.indexes.retain(|i| i.name != index_name);
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}

fn parse_column_definition(pair: pest::iterators::Pair<Rule>) -> Column {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().trim_matches('`').to_string();
    let data_type = parse_data_type(inner.next().unwrap());
    let mut column = Column::new(name, data_type);

    for constraint in inner {
        match constraint.as_str() {
            "NOT NULL" => column.nullable = false,
            "NULL" => column.nullable = true,
            s if s.starts_with("DEFAULT") => {
                column.default = Some(
                    s.strip_prefix("DEFAULT ")
                        .unwrap()
                        .trim_matches('\'')
                        .to_string(),
                )
            }
            "AUTO_INCREMENT" => column.auto_increment = true,
            "PRIMARY KEY" => {}
            _ => {}
        }
    }

    column
}

fn parse_index_definition(pair: pest::iterators::Pair<Rule>) -> Index {
    let mut inner = pair.into_inner();
    let index_type = inner.next().unwrap().as_str();
    let name = if index_type != "PRIMARY KEY" {
        inner
            .next()
            .map(|p| p.as_str().trim_matches('`').to_string())
            .unwrap_or_else(|| format!("index_{}", uuid::Uuid::new_v4()))
    } else {
        "PRIMARY".to_string()
    };
    let columns: Vec<String> = inner
        .map(|col| col.as_str().trim_matches('`').to_string())
        .collect();
    let unique = index_type.contains("UNIQUE") || index_type == "PRIMARY KEY";

    Index::new(name, columns, unique)
}

fn parse_data_type(pair: pest::iterators::Pair<Rule>) -> DataType {
    let type_name = pair
        .as_str()
        .split('(')
        .next()
        .unwrap()
        .trim()
        .to_uppercase();
    let mut inner = pair.into_inner();

    match type_name.as_str() {
        "TINYINT" | "SMALLINT" | "MEDIUMINT" | "INT" | "INTEGER" | "BIGINT" | "BIT" => {
            let size = inner.next().map(|p| p.as_str().parse::<u32>().unwrap());
            match type_name.as_str() {
                "TINYINT" => DataType::TinyInt(size),
                "SMALLINT" => DataType::SmallInt(size),
                "MEDIUMINT" => DataType::MediumInt(size),
                "INT" | "INTEGER" => DataType::Int(size),
                "BIGINT" => DataType::BigInt(size),
                "BIT" => DataType::Bit(size),
                _ => unreachable!(),
            }
        }
        "DECIMAL" | "NUMERIC" | "FLOAT" | "DOUBLE" => {
            let precision = inner.next().map(|p| p.as_str().parse::<u32>().unwrap());
            let scale = inner.next().map(|p| p.as_str().parse::<u32>().unwrap());
            match type_name.as_str() {
                "DECIMAL" | "NUMERIC" => {
                    DataType::Decimal(precision.and_then(|p| scale.map(|s| (p, s))))
                }
                "FLOAT" => DataType::Float(precision.and_then(|p| scale.map(|s| (p, s)))),
                "DOUBLE" => DataType::Double(precision.and_then(|p| scale.map(|s| (p, s)))),
                _ => unreachable!(),
            }
        }
        "DATE" => DataType::Date,
        "DATETIME" | "TIMESTAMP" | "TIME" | "YEAR" => {
            let size = inner.next().map(|p| p.as_str().parse::<u32>().unwrap());
            match type_name.as_str() {
                "DATETIME" => DataType::DateTime(size),
                "TIMESTAMP" => DataType::Timestamp(size),
                "TIME" => DataType::Time(size),
                "YEAR" => DataType::Year(size),
                _ => unreachable!(),
            }
        }
        "CHAR" | "VARCHAR" | "BINARY" | "VARBINARY" => {
            let size = inner
                .next()
                .map(|p| p.as_str().parse::<u32>().unwrap())
                .unwrap();
            match type_name.as_str() {
                "CHAR" => DataType::Char(Some(size)),
                "VARCHAR" => DataType::Varchar(Some(size)),
                "BINARY" => DataType::Binary(Some(size)),
                "VARBINARY" => DataType::Varbinary(Some(size)),
                _ => unreachable!(),
            }
        }
        "TINYBLOB" => DataType::TinyBlob,
        "BLOB" => DataType::Blob,
        "MEDIUMBLOB" => DataType::MediumBlob,
        "LONGBLOB" => DataType::LongBlob,
        "TINYTEXT" => DataType::TinyText,
        "TEXT" => DataType::Text,
        "MEDIUMTEXT" => DataType::MediumText,
        "LONGTEXT" => DataType::LongText,
        "ENUM" => {
            let values: Vec<String> = inner
                .map(|p| p.as_str().trim_matches('\'').to_string())
                .collect();
            DataType::Enum(values)
        }
        "SET" => {
            let values: Vec<String> = inner
                .map(|p| p.as_str().trim_matches('\'').to_string())
                .collect();
            DataType::Set(values)
        }
        "GEOMETRY" => DataType::Geometry,
        "POINT" => DataType::Point,
        "LINESTRING" => DataType::LineString,
        "POLYGON" => DataType::Polygon,
        "MULTIPOINT" => DataType::MultiPoint,
        "MULTILINESTRING" => DataType::MultiLineString,
        "MULTIPOLYGON" => DataType::MultiPolygon,
        "GEOMETRYCOLLECTION" => DataType::GeometryCollection,
        "JSON" => DataType::JSON,
        _ => unimplemented!("Data type {} not implemented", type_name),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_database() {
        let input = "CREATE DATABASE `test_db`;";
        let result = parse_mysqldump(input).unwrap();
        assert_eq!(result.len(), 1, "Expected 1 database, got {}", result.len());
        if !result.is_empty() {
            assert_eq!(result[0].name, "test_db", "Database name mismatch");
            assert!(result[0].tables.is_empty(), "Expected no tables");
            assert!(
                result[0].set_variables.is_empty(),
                "Expected no set variables"
            );
        }
    }

    #[test]
    fn test_create_table() {
        let input = r#"
        --
        -- Table structure for table `config`
        --
        CREATE DATABASE `test_db`;
        USE `test_db`;
        CREATE TABLE IF NOT EXISTS `config` (
          `name` varchar(255) NOT NULL,
          `value` text NOT NULL,
          PRIMARY KEY  (`name`)
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
        "#;
        let result = parse_mysqldump(input).unwrap();
        assert_eq!(result.len(), 1, "Expected 1 database, got {}", result.len());
        if !result.is_empty() {
            assert_eq!(result[0].name, "test_db", "Database name mismatch");
        }
        let db = result[0].clone();
        assert_eq!(
            db.tables.len(),
            1,
            "Expected 1 table, got {}",
            db.tables.len()
        );
        if let Some(ref table) = db.tables.get("config") {
            assert_eq!(
                table.columns.len(),
                2,
                "Expected 2 columns, got {}",
                table.columns.len()
            );
        }
    }

    #[test]
    fn test_create_table_with_primary_key() {
        let sql = r#"
        --
        -- Table structure for table `dns_record_types`
        --
        CREATE DATABASE `test_db`;
        USE `test_db`;
        
        CREATE TABLE IF NOT EXISTS `dns_record_types` (
          `id` int(10) unsigned NOT NULL auto_increment,
          `type` varchar(6) NOT NULL,
          `user_selectable` tinyint(1) NOT NULL default '0',
          PRIMARY KEY  (`id`)
        ) ENGINE=InnoDB  DEFAULT CHARSET=utf8 AUTO_INCREMENT=8 ;
        "#;
        let result = parse_mysqldump(sql).unwrap();
        assert_eq!(result.len(), 1, "Expected 1 database, got {}", result.len());
        if !result.is_empty() {
            assert_eq!(result[0].name, "test_db", "Database name mismatch");
        }
        let db = result[0].clone();
        assert_eq!(
            db.tables.len(),
            1,
            "Expected 1 table, got {}",
            db.tables.len()
        );
        if let Some(ref table) = db.tables.get("dns_record_types") {
            assert_eq!(
                table.columns.len(),
                3,
                "Expected 3 columns, got {}",
                table.columns.len()
            );
        }
    }

    #[test]
    fn test_insert_into_table() {
        let input = r#"
        CREATE DATABASE `test_db`;
        USE `test_db`;
        CREATE TABLE `users` (
            `id` INT NOT NULL AUTO_INCREMENT,
            `name` VARCHAR(255) NOT NULL,
            `email` VARCHAR(255) NOT NULL,
            PRIMARY KEY (`id`)
        );
        INSERT INTO `users` (`name`, `email`) VALUES ('John Doe', 'john.doe@example.com');
        "#;
        let result = parse_mysqldump(input).unwrap();
        assert_eq!(result.len(), 1);
        let db = &result[0];
        assert_eq!(db.name, "test_db");
        assert_eq!(db.tables.keys().len(), 1);
        let table = db.tables.get("users").unwrap();
        assert_eq!(table.columns.len(), 3);
        assert_eq!(table.inserts.len(), 1);
    }

    #[test]
    fn test_update_record_in_table() {
        let my_parser = get_test_database_and_table();
        let input = "UPDATE `users` SET `name` = 'Jane Doe' WHERE `id` = 1;";
        let parsed = my_parser.parse(input).unwrap();
        let databases = parsed.get_databases();
        assert_eq!(databases.len(), 1);
        let db = &databases[0];
        assert_eq!(db.name, "test_db");
        assert_eq!(db.tables.keys().len(), 1);
        let table = db.tables.get("users").unwrap();
        assert_eq!(table.updates.len(), 1);
    }

    #[test]
    fn test_multiple_statements() {
        let input = r#"
        CREATE DATABASE `test_db`;
        USE `test_db`;
        CREATE TABLE `users` (
            `id` INT NOT NULL AUTO_INCREMENT,
            `name` VARCHAR(255) NOT NULL,
            PRIMARY KEY (`id`)
        );
        INSERT INTO `users` (`name`) VALUES ('John Doe');
        UPDATE `users` SET `name` = 'Jane Doe' WHERE `id` = 1;
        DELETE FROM `users` WHERE `id` = 1;
        SET @last_id = 1;
        "#;
        let result = parse_mysqldump(input).unwrap();
        assert_eq!(result.len(), 1);
        let db = &result[0];
        assert_eq!(db.name, "test_db");
        println!("db: {:?}", db);
        assert_eq!(db.tables.keys().len(), 1);
        let table = db.tables.get("users").unwrap();
        assert_eq!(table.columns.len(), 2);
        assert_eq!(table.inserts.len(), 1);
        assert_eq!(table.updates.len(), 1);
        assert_eq!(table.deletes.len(), 1);
        assert_eq!(db.set_variables.len(), 1);
    }

    fn get_test_database_and_table() -> MyParser {
        let input = r#"
        CREATE DATABASE `test_db`;
        USE `test_db`;
        CREATE TABLE `users` (
            `id` INT NOT NULL AUTO_INCREMENT,
            `name` VARCHAR(255) NOT NULL,
            `email` VARCHAR(255) NOT NULL,
            `password` VARCHAR(255) NOT NULL,
            PRIMARY KEY (`id`)
        );
        "#;
        let mut my_parser = MyParser::new();
        my_parser = my_parser.parse(input).unwrap();
        my_parser
    }
}