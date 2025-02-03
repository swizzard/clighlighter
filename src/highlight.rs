use std::fmt::Write;
use tree_sitter::{Language, Node, Parser, Point, TreeCursor};

pub fn highlight<H: Highlight>(h: H, input: &str) -> String {
    let mut parser = Parser::new();
    parser
        .set_language(&h.language())
        .expect("Error loading Typescript grammar");
    let tree = parser.parse(input, None).unwrap();
    let mut output = String::from("<pre class=\"code\">\n");
    let mut cursor = tree.walk();
    let mut more = cursor.goto_first_child();
    while more {
        more = handle_statement(&h, &mut cursor, &mut output, input, None);
    }
    output.push_str("\n</pre>");
    output
}

fn handle_statement<H: Highlight>(
    h: &H,
    cursor: &mut TreeCursor<'_>,
    output: &mut String,
    input: &str,
    prev_end: Option<Point>,
) -> bool {
    let n = cursor.node();
    write!(output, "{}", h.highlight_node(&n, input, prev_end)).expect("can't write");
    let pe = if h.is_printed_node(&n, input) {
        Some(n.end_position())
    } else {
        prev_end
    };

    if next_more(cursor, false) {
        handle_statement(h, cursor, output, input, pe)
    } else {
        false
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

pub trait Highlight {
    fn language(&self) -> Language;
    fn is_printed_node(&self, node: &Node, input: &str) -> bool;
    fn highlight_node(&self, node: &Node, input: &str, prev_end: Option<Point>) -> String;
}
