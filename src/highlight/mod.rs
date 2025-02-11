//! # highlight
//!
//! the core functionality of the app is
//! - the [`Highlight`] trait and
//! - tree traversal via [`next_more`]
//!
pub mod explore;
pub mod rust;
mod shared;
pub mod ts;

use std::io::{self, Write};
use tree_sitter::{Language, Node, Parser, Point, TreeCursor};

pub trait Highlight {
    /// the tree-sitter [`Language`] used by this highlighter
    fn language(&self) -> Language;
    /// turns a [`Node`] into highlighted text
    fn highlight_node(&self, node: &Node, input: &[u8], prev_end: Option<Point>) -> Option<String>;
    fn should_highlight_children(&self, node: &Node) -> bool;
}

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
        more = handle_statement(h, &mut cursor, output, input.as_ref(), None)?;
    }
    output.write_all(b"\n</pre>")?;
    Ok(())
}

fn handle_statement(
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
        handle_statement(h, cursor, output, input, next_end)
    } else {
        Ok(false)
    }
}

fn next_more(cursor: &mut TreeCursor<'_>, should_highlight_children: bool) -> bool {
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
