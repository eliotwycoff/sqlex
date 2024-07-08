use crate::parser::{
    statements::TEMPLATES,
    types::{Column, ForeignKey, Index, PrimaryKey, TableOption},
    Rule, Sql,
};
use pest::iterators::Pair;

#[derive(Debug, Clone)]
pub struct CreateTable {
    pub name: String,
    pub columns: Vec<Column>,
    pub primary_key: Option<PrimaryKey>,
    pub foreign_keys: Vec<ForeignKey>,
    pub indexes: Vec<Index>,
    pub options: Vec<TableOption>,
}

impl From<Pair<'_, Rule>> for CreateTable {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let mut inner = pair.into_inner();
        let name = inner
            .next()
            .expect("table name")
            .as_str()
            .trim_matches('`')
            .to_string();
        let mut columns = Vec::new();
        let mut primary_key = None;
        let mut foreign_keys = Vec::new();
        let mut indexes = Vec::new();
        let mut options = Vec::new();

        inner
            .next()
            .expect("table specs")
            .into_inner()
            .for_each(|spec| match spec.as_rule() {
                Rule::COLUMN_DEFINITION => columns.push(Column::from(spec)),
                Rule::PRIMARY_KEY => primary_key = Some(PrimaryKey::from(spec)),
                Rule::FOREIGN_KEY => foreign_keys.push(ForeignKey::from(spec)),
                Rule::INDEX_DEFINITION => indexes.push(Index::from(spec)),
                other => panic!("{other:?} is not a valid table column spec"),
            });
        inner
            .next()
            .expect("table options")
            .into_inner()
            .for_each(|opt| options.push(TableOption::from(opt)));

        Self {
            name,
            columns,
            primary_key,
            foreign_keys,
            indexes,
            options,
        }
    }
}

impl Sql for CreateTable {
    fn as_sql(&self) -> String {
        let mut ctx = tera::Context::new();
        let mut table_specs = self
            .columns
            .iter()
            .map(|col| col.as_sql())
            .collect::<Vec<String>>();
        let table_options = self
            .options
            .iter()
            .map(|col| col.as_sql())
            .collect::<Vec<String>>();

        self.primary_key
            .as_ref()
            .inspect(|pk| table_specs.push(pk.as_sql()));
        self.foreign_keys
            .iter()
            .for_each(|fk| table_specs.push(fk.as_sql()));
        self.indexes
            .iter()
            .for_each(|idx| table_specs.push(idx.as_sql()));

        ctx.insert("name", self.name.as_str());
        ctx.insert("table_specs", &table_specs);
        ctx.insert("table_options", &table_options);

        TEMPLATES
            .render("create_table/template.sql", &ctx)
            .expect("Failed to render create table sql")
            .trim()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::{types::DataType, MySqlParser};
    use pest::Parser;

    #[test]
    fn can_parse_create_table() {
        let create_table = CreateTable::from(
            MySqlParser::parse(
                Rule::CREATE_TABLE,
                "CREATE TABLE `application` (
                    `Id` int NOT NULL AUTO_INCREMENT,
                    `ProductId` int NOT NULL DEFAULT '0',
                    `Name` varchar(36) NOT NULL,
                    `SecurityToken` varchar(200) DEFAULT NULL,
                    `RoutingKey` varchar(50) DEFAULT NULL,
                    `AdminPrivilege` tinyint(1) DEFAULT '0',
                    `CreatedAt` datetime NOT NULL,
                    PRIMARY KEY (`Id`),
                    KEY `fk_application_product` (`ProductId`),
                    CONSTRAINT `fk_application_product` FOREIGN KEY (`ProductId`) REFERENCES `product` (`Id`)
                ) ENGINE=InnoDB AUTO_INCREMENT=101 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;"
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input")
        );

        assert_eq!(create_table.name.as_str(), "application");
        assert_eq!(create_table.columns.len(), 7);
        assert!(create_table.primary_key.is_some());
        assert_eq!(create_table.foreign_keys.len(), 1);
        assert_eq!(create_table.indexes.len(), 1);
        assert_eq!(create_table.options.len(), 4);
    }

    #[test]
    fn can_write_create_table() {
        assert_eq!(
            CreateTable {
                name: String::from("application"),
                columns: vec![
                    Column {
                        name: String::from("Id"),
                        data_type: DataType::Int {
                            m: None,
                            unsigned: false,
                            zerofill: false
                        },
                        nullable: false,
                        default: None,
                        auto_increment: true,
                        comment: None,
                    },
                    Column {
                        name: String::from("ProductId"),
                        data_type: DataType::Int {
                            m: None,
                            unsigned: false,
                            zerofill: false
                        },
                        nullable: false,
                        default: Some(String::from("0")),
                        auto_increment: false,
                        comment: None,
                    },
                    Column {
                        name: String::from("Name"),
                        data_type: DataType::Varchar {
                            m: Some(36),
                            charset_name: None,
                            collation_name: None
                        },
                        nullable: false,
                        default: None,
                        auto_increment: false,
                        comment: None,
                    },
                    Column {
                        name: String::from("SecurityToken"),
                        data_type: DataType::Varchar {
                            m: Some(200),
                            charset_name: None,
                            collation_name: None
                        },
                        nullable: true,
                        default: Some(String::from("NULL")),
                        auto_increment: false,
                        comment: None,
                    },
                    Column {
                        name: String::from("RoutingKey"),
                        data_type: DataType::Varchar {
                            m: Some(50),
                            charset_name: None,
                            collation_name: None
                        },
                        nullable: true,
                        default: Some(String::from("NULL")),
                        auto_increment: false,
                        comment: None,
                    },
                    Column {
                        name: String::from("AdminPrivilege"),
                        data_type: DataType::TinyInt {
                            m: Some(1),
                            unsigned: false,
                            zerofill: false
                        },
                        nullable: true,
                        default: Some(String::from("0")),
                        auto_increment: false,
                        comment: None,
                    },
                    Column {
                        name: String::from("CreatedAt"),
                        data_type: DataType::DateTime { fsp: None },
                        nullable: false,
                        default: None,
                        auto_increment: false,
                        comment: None,
                    },
                ],
                primary_key: Some(PrimaryKey {
                    name: None,
                    column_names: vec![String::from("Id")]
                }),
                foreign_keys: vec![ForeignKey {
                    name: Some(String::from("fk_application_product")),
                    local_column_names: vec![String::from("ProductId")],
                    foreign_column_names: vec![String::from("Id")],
                    foreign_table_name: String::from("product")
                },],
                indexes: vec![Index {
                    name: String::from("fk_application_product"),
                    columns: vec![String::from("ProductId")],
                    unique: false
                },],
                options: vec![
                    TableOption::Engine {
                        value: String::from("InnoDB")
                    },
                    TableOption::CharacterSet {
                        default: true,
                        value: String::from("utf8mb4")
                    },
                    TableOption::Collate {
                        default: false,
                        value: String::from("utf8mb4_0900_ai_ci")
                    },
                ],
            }
            .as_sql()
            .trim(),
            "CREATE TABLE `application` (\n  `Id` INT NOT NULL AUTO_INCREMENT,\n  `ProductId` INT NOT NULL DEFAULT '0',\n  `Name` VARCHAR (36) NOT NULL,\n  `SecurityToken` VARCHAR (200) DEFAULT NULL,\n  `RoutingKey` VARCHAR (50) DEFAULT NULL,\n  `AdminPrivilege` TINYINT (1) DEFAULT '0',\n  `CreatedAt` DATETIME NOT NULL,\n  PRIMARY KEY (`Id`),\n  CONSTRAINT `fk_application_product` FOREIGN KEY (`ProductId`) REFERENCES `product` (`Id`),\n  KEY `fk_application_product` (`ProductId`)\n) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;"
        );
    }
}