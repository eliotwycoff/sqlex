use crate::parser::{
    types::{Column, Delete, Index, Insert, Update},
    Sql,
};

#[derive(Debug, Clone, Default)]
pub struct Object {
    // Table settings
    pub name: String,
    pub columns: Vec<Column>,
    pub primary_key: Option<String>,
    pub indexes: Vec<Index>,

    // Table options
    pub engine: Option<String>,
    pub auto_increment: Option<String>,
    pub charset: Option<String>,
    pub collate: Option<String>,

    // Row operations
    pub inserts: Vec<Insert>,
    pub updates: Vec<Update>,
    pub deletes: Vec<Delete>,
}

impl Object {
    pub fn new(name: String) -> Self {
        Self {
            name,
            columns: Vec::new(),
            primary_key: None,
            indexes: Vec::new(),
            engine: None,
            auto_increment: None,
            charset: None,
            collate: None,
            inserts: Vec::new(),
            updates: Vec::new(),
            deletes: Vec::new(),
        }
    }
}

impl Sql for Object {
    fn as_sql(&self) -> String {
        let columns = self
            .columns
            .iter()
            .map(|col| col.as_sql())
            .collect::<Vec<String>>()
            .join(",\n");

        // TODO: Finish this.
        let engine = String::from("");
        let auto_increment = String::from("");
        let charset = String::from("");
        let collate = String::from("");

        // TODO: Finish this
        format!(
            r#"
            --
            -- Table structure for table `{}` 
            --

            DROP TABLE IF EXISTS `{}`;
            CREATE TABLE `{}` (
              -- TODO: FINISH THIS
            ) {} {} {} {}
            "#,
            self.name, self.name, self.name, engine, auto_increment, charset, collate,
        )
    }
}
