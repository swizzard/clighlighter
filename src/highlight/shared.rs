//! shared utility functions for highlighters
use tree_sitter::{Node, Point};

/// helper function that calculates leading whitespace based on the start of the current [`Node`]
/// and the end of the previous node
pub fn initial_padding(node: &Node, prev_end: Option<Point>) -> String {
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

/// helper function to extract the text associated with a [`Node`]
pub fn node_text<'a>(node: &'a tree_sitter::Node, input: &'a [u8]) -> &'a str {
    node.utf8_text(input).expect("non-utf8 input")
}
