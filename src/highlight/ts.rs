use crate::highlight::Highlight;
use crate::shared::{initial_padding, node_text};
use std::collections::HashSet;
use std::fmt::Write;
use std::sync::LazyLock;
use tree_sitter::{Language, Node, Point};

static TS_SPECIAL: LazyLock<HashSet<&str>> = LazyLock::new(|| {
    HashSet::from([
        "as",
        "const",
        "export",
        "function",
        "identifier",
        "interface",
        "literal_type",
        "new",
        "number",
        "property_identifier",
        "return",
        "type_identifier",
        "undefined",
    ])
});

static TS_REGULAR: LazyLock<HashSet<&str>> = LazyLock::new(|| {
    HashSet::from([
        "=", "<", ">", "(", ")", "{", "}", "|", ";", ",", "=>", "===", ":", "?", ".",
    ])
});

pub struct TS;

impl TS {
    fn is_special_node(k: &str, txt: &str) -> bool {
        if k == "literal_type" && txt == "undefined" {
            false
        } else {
            TS_SPECIAL.contains(k)
        }
    }

    fn is_regular_node(k: &str) -> bool {
        TS_REGULAR.contains(k)
    }
}

impl Highlight for TS {
    fn language(&self) -> Language {
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()
    }
    fn highlight_node(&self, node: &Node, input: &str, prev_end: Option<Point>) -> String {
        let k = node.kind();
        let txt = node_text(node, input);
        if TS::is_special_node(k, txt) {
            let mut s = initial_padding(node, prev_end);
            write!(&mut s, "<code class=\"{}\">{}</code>", k, txt).expect("can't write");
            s
        } else if TS::is_regular_node(k) {
            let mut s = initial_padding(node, prev_end);
            s.push_str(txt);
            s
        } else {
            String::new()
        }
    }
}
