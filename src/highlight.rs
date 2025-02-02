use std::fmt::Write;
use tree_sitter::{Node, Parser, Point, TreeCursor};

pub fn highlight(input: &str) -> String {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
        .expect("Error loading Typescript grammar");
    let tree = parser.parse(input, None).unwrap();
    let mut output = String::from("<pre class=\"code\">\n");
    let mut cursor = tree.walk();
    let mut more = cursor.goto_first_child();
    while more {
        more = handle_statement(&mut cursor, &mut output, input, None);
    }
    output.push_str("\n</pre>");
    output
}

fn handle_statement(
    cursor: &mut TreeCursor<'_>,
    output: &mut String,
    input: &str,
    prev_end: Option<Point>,
) -> bool {
    let n = cursor.node();
    write!(output, "{}", pretty_node(n, input, prev_end)).expect("can't write");
    let k = n.kind();
    let txt = n.utf8_text(input.as_bytes()).expect("non-utf8 input");
    let pe = if is_printed_node(k, txt) {
        Some(n.end_position())
    } else {
        prev_end
    };

    if next_more(cursor, false) {
        handle_statement(cursor, output, input, pe)
    } else {
        false
    }
}

fn pretty_node(node: Node, input: &str, prev_end: Option<Point>) -> String {
    let k = node.kind();
    let txt = node.utf8_text(input.as_bytes()).expect("non-utf8 input");
    if is_special_node(k, txt) {
        let mut s = initial_padding(&node, prev_end);
        write!(&mut s, "<code class=\"{}\">{}</code>", k, txt).expect("can't write");
        s
    } else if is_regular_node(k) {
        let mut s = initial_padding(&node, prev_end);
        s.push_str(txt);
        s
    } else {
        String::new()
    }
}

fn is_special_node(k: &str, txt: &str) -> bool {
    if k == "literal_type" && txt == "undefined" {
        false
    } else {
        matches!(
            k,
            "new"
                | "as"
                | "const"
                | "export"
                | "interface"
                | "type_identifier"
                | "property_identifier"
                | "literal_type"
                | "function"
                | "return"
                | "number"
                | "identifier"
                | "undefined"
        )
    }
}

fn is_regular_node(k: &str) -> bool {
    matches!(
        k,
        "=" | "<" | ">" | "(" | ")" | "{" | "}" | "|" | ";" | "," | "=>" | "===" | ":" | "?" | "."
    )
}

fn is_printed_node(k: &str, txt: &str) -> bool {
    is_special_node(k, txt) || is_regular_node(k)
}

fn initial_padding(node: &Node, prev_end: Option<Point>) -> String {
    let mut s = String::new();
    let Point {
        row: new_row,
        column: new_column,
    } = node.start_position();
    if let Some(Point {
        row: prev_row,
        column: prev_column,
    }) = prev_end
    {
        let row_diff = new_row.saturating_sub(prev_row);
        let col_diff = if row_diff == 0 {
            // same row, keep going
            new_column.saturating_sub(prev_column)
        } else {
            // new row, start fresh
            new_column
        };
        for _ in 0..row_diff {
            s.push('\n');
        }
        for _ in 0..col_diff {
            s.push(' ');
        }
    }
    s
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
