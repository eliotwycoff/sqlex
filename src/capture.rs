use serde::{Deserialize, Serialize};
pub type Range = (usize, usize, usize, usize);

/// Get the text of a child node
///
/// Returns an empty string if the child does not exists, or the text could
/// not be obtained
pub fn child_text<'tree>(
    node: tree_sitter::Node<'tree>,
    name: &str,
    code: &'tree [u8],
) -> &'tree str {
    node.child_by_field_name(name)
        .and_then(|child| child.utf8_text(code).ok())
        .unwrap_or("")
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CaptureKind {
    #[default]
    Unknown,
    Dependency(String),
}

#[derive(Default, Debug, Clone)]
pub struct SimpleCapture {
    pub name: String,
    pub text: String,
}

/// A capture resulting from a `tree-sitter` query
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Capture<'tree> {
    #[allow(dead_code)]
    /// The index of the capture in the pattern
    index: u32,

    /// The name of the capture in the pattern
    pub name: String,

    /// The captured node
    #[serde(skip)]
    pub node: tree_sitter::Node<'tree>,

    /// The captured range
    pub range: Range,

    /// The captured text
    pub text: String,

    /// The capture kind
    pub kind: CaptureKind,
}

impl<'tree> Capture<'tree> {
    pub fn new(
        index: u32,
        name: String,
        node: tree_sitter::Node<'tree>,
        range: Range,
        text: String,
        kind: CaptureKind,
    ) -> Capture {
        Capture {
            index,
            name,
            node,
            range,
            text,
            kind,
        }
    }
}
