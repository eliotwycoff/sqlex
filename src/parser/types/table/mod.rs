use crate::parser::{
    types::{Column, Delete, Index, Insert, PrimaryKey, Update, TEMPLATES},
    Rule, Sql,
};
use pest::iterators::Pair;
use tera::Context;

#[derive(Debug, Clone, Default)]
pub struct Table {
    // Table settings
    pub name: String,
    pub columns: Vec<Column>,
    pub primary_key: Option<PrimaryKey>,
    pub indexes: Vec<Index>,

    // Table options
    pub auto_increment: Option<String>,
    pub charset: Option<String>,
    pub collate: Option<String>,
    pub comment: Option<String>,
    pub engine: Option<String>,
    pub row_format: Option<String>,
    pub stats_persistent: Option<String>,

    // Row operations
    pub inserts: Vec<Insert>,
    pub updates: Vec<Update>,
    pub deletes: Vec<Delete>,
}

impl Table {
    pub fn new(name: String) -> Self {
        Self {
            name,
            columns: Vec::new(),
            primary_key: None,
            indexes: Vec::new(),
            auto_increment: None,
            charset: None,
            collate: None,
            comment: None,
            engine: None,
            row_format: None,
            stats_persistent: None,
            inserts: Vec::new(),
            updates: Vec::new(),
            deletes: Vec::new(),
        }
    }
}

impl From<Pair<'_, Rule>> for Table {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let mut inner = pair.into_inner();
        let table_name = inner
            .next()
            .expect("unable to extract table name")
            .as_str()
            .trim_matches('`')
            .to_string();
        let mut table = Self::new(table_name);

        for element in inner {
            match element.as_rule() {
                Rule::COLUMN_DEFINITION => {
                    let column = Column::from(element);

                    table.columns.push(column);

                    // TODO: Check if this column is marked as a PRIMARY KEY
                }
                Rule::PRIMARY_KEY => {
                    table.primary_key = Some(PrimaryKey::from(element));
                }
                Rule::INDEX_DEFINITION => {
                    table.indexes.push(Index::from(element));
                }
                _ => {}
            }
        }

        table
    }
}

impl Sql for Table {
    fn as_sql(&self) -> String {
        let mut ctx = Context::new();

        ctx.insert("name", self.name.as_str());

        let mut column_specifications = self
            .columns
            .iter()
            .map(|col| col.as_sql())
            .collect::<Vec<String>>();

        self.primary_key
            .as_ref()
            .inspect(|pk| column_specifications.push(pk.as_sql()));
        self.indexes
            .iter()
            .for_each(|index| column_specifications.push(index.as_sql()));

        ctx.insert("column_specifications", &column_specifications);

        // Insert table options into the context.
        ctx.insert("auto_increment", &self.auto_increment);
        ctx.insert("charset", &self.charset);
        ctx.insert("collate", &self.collate);
        ctx.insert("column", &self.comment);
        ctx.insert("engine", &self.engine);
        ctx.insert("row_format", &self.row_format);
        ctx.insert("stats_persistent", &self.stats_persistent);

        TEMPLATES
            .render("table", &ctx)
            .expect("Failed to render table sql")
    }
}
