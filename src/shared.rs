use tree_sitter::{Node, Point};
pub(crate) fn initial_padding(node: &Node, prev_end: Option<Point>) -> String {
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

pub(crate) fn node_text<'a>(node: &'a tree_sitter::Node, input: &'a str) -> &'a str {
    node.utf8_text(input.as_bytes()).expect("non-utf8 input")
}
