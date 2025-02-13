//! # the core functionality of the app
//!
pub mod explore;
pub mod rust;
pub mod shared;
pub mod ts;

use std::io::{self, Write};
use tree_sitter::{Language, Node, Parser, Point, TreeCursor};

/// the core trait of the app
///
/// note that the [`tree_sitter::Language`] exposed via `language` is a value and not an
/// associated type. defining multiple `Highlight` implementations for the same [`Language`]
/// is not only possible but encourged.
pub trait Highlight {
    /// the tree-sitter [`Language`] used by this highlighter
    fn language(&self) -> Language;
    /// turns a [`Node`] into highlighted text    
    /// note that returning `Some(String::new())` will result in wonky whitespace behavior    
    /// you have access to
    /// - the [`tree_sitter::Node`]
    /// - the text being highlit, which is mostly useful for extracting node text with [`shared::node_text`]
    /// - the [`Point`] at which the previous highlighted span ended, which is
    ///   useful for whitespace management with [`shared::initial_padding`]
    fn highlight_node(&self, node: &Node, input: &[u8], prev_end: Option<Point>) -> Option<String>;
    /// _based on this node alone_, should we descend into its children or move on to its
    /// siblings?
    fn should_highlight_children(&self, node: &Node) -> bool;
}

/// use a [`Highlight`] implementation to highlight some code and write it to `output`
pub fn highlight<I>(h: &dyn Highlight, input: &I, output: &mut dyn Write) -> io::Result<()>
where
    I: AsRef<[u8]>,
{
    let mut parser = Parser::new();
    parser
        .set_language(&h.language())
        .expect("Error loading grammar");
    let tree = parser.parse(input, None).unwrap();
    output.write_all(b"<pre class=\"code\">\n")?;
    let mut cursor = tree.walk();
    let mut more = cursor.goto_first_child();
    while more {
        more = handle_node(h, &mut cursor, output, input.as_ref(), None)?;
    }
    output.write_all(b"\n</pre>")?;
    Ok(())
}

/// process a node, writing any resulting highlighted code to `output` before attempting to move
/// the `cursor` to the next node
fn handle_node(
    h: &dyn Highlight,
    cursor: &mut TreeCursor<'_>,
    output: &mut dyn Write,
    input: &[u8],
    prev_end: Option<Point>,
) -> io::Result<bool> {
    let n = cursor.node();
    let next_end = if let Some(highlit) = h.highlight_node(&n, input, prev_end) {
        output.write_all(highlit.as_bytes())?;
        Some(n.end_position())
    } else {
        prev_end
    };
    if next_more(cursor, h.should_highlight_children(&n)) {
        handle_node(h, cursor, output, input, next_end)
    } else {
        Ok(false)
    }
}

/// traversal function - depth-first but a highlighter has partial control via
/// [`Highlight::should_highlight_children`], the result of which is passed in here
fn next_more(cursor: &mut TreeCursor<'_>, should_highlight_children: bool) -> bool {
    // we try to move the cursor to the "next node", which is generally
    // first child || next sibling || parent
    // `should_highlight_children` being `false` treats a node as childless
    let mut more = if !should_highlight_children {
        false
    } else {
        cursor.goto_first_child()
    };
    if !more {
        more = cursor.goto_next_sibling();
    };
    if !more {
        if !cursor.goto_parent() {
            return false;
        }
        more = next_more(cursor, false);
    };
    more
}
