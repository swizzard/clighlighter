use crate::highlight::Highlight;
use crate::shared::{initial_padding, node_text};
use std::collections::HashSet;
use std::fmt::Write;
use std::sync::LazyLock;
use tree_sitter::{Language, Node, Point};

static RUST_SPECIAL: LazyLock<HashSet<&str>> = LazyLock::new(|| {
    HashSet::from([
        "&",
        "fn",
        "pub",
        "mod",
        "identifier",
        "use",
        "type_identifier",
        "primitive_type",
        "let",
        "field_identifier",
        "string_literal",
        "mutable_specifier",
        "while",
        "if",
        "else",
        "boolean_literal", // NOT "false", "true"
        "trait",
        "self",
        "enum",
        "line_comment",
        "struct",
        "crate",
        "static",
        "impl",
        "for",
    ])
});
static RUST_REGULAR: LazyLock<HashSet<&str>> = LazyLock::new(|| {
    HashSet::from([
        "(", ")", ";", "::", "{", "}", ",", ":", "=", ".", "<", ">", "'", "_", "!", "->", "#", "[",
        "]",
    ])
});

pub struct Rust;

impl Rust {
    fn is_special_node(k: &str) -> bool {
        RUST_SPECIAL.contains(k)
    }
    fn is_regular_node(k: &str) -> bool {
        RUST_REGULAR.contains(k)
    }
}

impl Highlight for Rust {
    fn language(&self) -> Language {
        tree_sitter_rust::LANGUAGE.into()
    }
    fn highlight_node(&self, node: &Node, input: &str, prev_end: Option<Point>) -> String {
        let k = node.kind();
        let txt = node_text(node, input);
        if Rust::is_special_node(k) {
            let mut s = initial_padding(node, prev_end);
            write!(&mut s, "<code class=\"{}\">{}</code>", k, txt).expect("can't write");
            s
        } else if Rust::is_regular_node(k) {
            let mut s = initial_padding(node, prev_end);
            s.push_str(txt);
            s
        } else {
            String::new()
        }
    }
}
