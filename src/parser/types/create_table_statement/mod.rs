use crate::parser::{
    types::{Column, ForeignKey, Index, PrimaryKey, TableOption, TEMPLATES},
    Rule, Sql,
};
use pest::iterators::Pair;
use tera::Context;

#[derive(Debug, Clone, Default)]
pub struct CreateTableStatement {
    pub name: String,
    pub columns: Vec<Column>,
    pub primary_key: Option<PrimaryKey>,
    pub foreign_keys: Vec<ForeignKey>,
    pub indexes: Vec<Index>,
    pub options: Vec<TableOption>,
}

impl CreateTableStatement {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
}

impl From<Pair<'_, Rule>> for CreateTableStatement {
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
                // Rule::TABLE_OPTIONS => {
                //     element.into_inner().for_each(|opt| {
                //         let s = opt.as_str().to_ascii_uppercase();

                //         if s.contains("AUTO_INCREMENT") {
                //             table.auto_increment = Some(opt.into_inner().next().expect("NUMBER").as_str().to_string());
                //         } else if s.contains("CHARSET") {
                //             table.charset = Some(opt.into_inner().next().expect(""));
                //         }
                //     });
                // }
                other => panic!(
                    "Expected COLUMN_DEFINITION, PRIMARY_KEY, INDEX_DEFINITION or TABLE_OPTIONS, not {other:?}"
                ),
            }
        }

        table
    }
}

impl Sql for CreateTableStatement {
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

        // // Insert table options into the context.
        // ctx.insert("auto_increment", &self.auto_increment);
        // ctx.insert("charset", &self.charset);
        // ctx.insert("collate", &self.collate);
        // ctx.insert("column", &self.comment);
        // ctx.insert("engine", &self.engine);
        // ctx.insert("row_format", &self.row_format);
        // ctx.insert("stats_persistent", &self.stats_persistent);

        TEMPLATES
            .render("table/template.sql", &ctx)
            .expect("Failed to render table sql")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::MySqlParser;
    use pest::Parser;
}
