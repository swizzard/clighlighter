use crate::highlight::Highlight;
use crate::initial_padding::initial_padding;
use std::fmt::Write;
use tree_sitter::{Language, Node, Point};

pub struct TS;

impl TS {
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
            "=" | "<"
                | ">"
                | "("
                | ")"
                | "{"
                | "}"
                | "|"
                | ";"
                | ","
                | "=>"
                | "==="
                | ":"
                | "?"
                | "."
        )
    }

    fn is_printed_node(k: &str, txt: &str) -> bool {
        TS::is_special_node(k, txt) || TS::is_regular_node(k)
    }
}

impl Highlight for TS {
    fn language(&self) -> Language {
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()
    }
    fn is_printed_node(&self, node: &Node, input: &str) -> bool {
        TS::is_printed_node(node.kind(), node.utf8_text(input.as_bytes()).unwrap())
    }
    fn highlight_node(&self, node: &Node, input: &str, prev_end: Option<Point>) -> String {
        let k = node.kind();
        let txt = node.utf8_text(input.as_bytes()).expect("non-utf8 input");
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
