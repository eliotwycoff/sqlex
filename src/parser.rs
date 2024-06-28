use std::sync::Mutex;

use crate::capture::{Capture, CaptureKind};

pub struct TreesitterParser {
    inner: Mutex<tree_sitter::Parser>,

    query: tree_sitter::Query,
}

impl TreesitterParser {
    pub fn new(query: &str) -> Self {
        let lang = tree_sitter_sql::language();
        let mut parser = tree_sitter::Parser::new();
        let _ = parser.set_language(lang);

        let query = tree_sitter::Query::new(lang, query); //.expect("Query should compile");
        let query = query.expect("Query should compile");

        Self {
            inner: Mutex::new(parser),
            query,
        }
    }

    pub fn parse(&self, code: &[u8]) -> tree_sitter::Tree {
        self.inner
            .lock()
            .expect("unable to lock parse")
            .parse(code, None)
            .expect("unable to parse tree result")
    }

    pub fn query<'tree>(
        &self,
        code: &[u8],
        tree: &'tree tree_sitter::Tree,
    ) -> Vec<(usize, Vec<Capture<'tree>>)> {
        let mut cursor = tree_sitter::QueryCursor::new();
        let matches = cursor.matches(&self.query, tree.root_node(), code);

        let capture_names = self.query.capture_names();
        matches
            .map(|query_match| {
                let pattern = query_match.pattern_index;
                let captures = query_match
                    .captures
                    .iter()
                    .map(|capture| {
                        let start = capture.node.start_position();
                        let end = capture.node.end_position();
                        Capture::new(
                            capture.index,
                            capture_names[capture.index as usize].to_string(),
                            capture.node,
                            (start.row, start.column, end.row, end.column),
                            capture
                                .node
                                .utf8_text(code)
                                .expect("Should be able to get text")
                                .to_string(),
                            CaptureKind::default(),
                        )
                    })
                    .collect();
                (pattern, captures)
            })
            .collect()
    }
}
