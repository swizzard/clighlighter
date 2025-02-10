pub mod explore;
pub mod rust;
pub mod ts;

use std::io::{self, Write};
use tree_sitter::{Language, Node, Parser, Point, TreeCursor};

pub trait Highlight {
    fn language(&self) -> Language;
    fn highlight_node(&self, node: &Node, input: &[u8], prev_end: Option<Point>) -> String;
}

pub fn highlight<I>(h: &dyn Highlight, input: &I, output: &mut dyn Write) -> io::Result<()>
where
    I: AsRef<[u8]>,
{
    let mut parser = Parser::new();
    parser
        .set_language(&h.language())
        .expect("Error loading Typescript grammar");
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
    let highlit = h.highlight_node(&n, input, prev_end);
    let pe = if !highlit.is_empty() {
        output.write_all(highlit.as_bytes())?;
        Some(n.end_position())
    } else {
        prev_end
    };
    if next_more(cursor, false) {
        handle_statement(h, cursor, output, input, pe)
    } else {
        Ok(false)
    }
}

fn next_more(cursor: &mut TreeCursor<'_>, skip_child: bool) -> bool {
    let mut more = if skip_child {
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
        more = next_more(cursor, true);
    };

    more
}
