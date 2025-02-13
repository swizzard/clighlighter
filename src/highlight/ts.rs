//! highlighter for TypeScript code
use crate::highlight::Highlight;
use crate::highlight::shared::{initial_padding, node_text};
use std::collections::HashSet;
use std::fmt::Write;
use std::sync::LazyLock;
use tree_sitter::{Language, Node, Point};

// I'm not sure if this LazyLock + static methods thing is a good pattern or not
// but I'm going with it

/// "special" [`Node::kind`]s that we highlight
static TS_SPECIAL: LazyLock<HashSet<&str>> = LazyLock::new(|| {
    HashSet::from([
        "any",
        "as",
        "assert",
        "async",
        "await",
        "boolean",
        "case",
        "catch",
        "class",
        "comment",
        "continue",
        "const",
        "debugger",
        "declare",
        "delete",
        "do",
        "else",
        "enum",
        "export",
        "extends",
        "finally",
        "for",
        "from",
        "function",
        "get",
        "global",
        "html_comment",
        "identifier",
        "if",
        "implements",
        "import",
        "in",
        "infer",
        "instanceof",
        "interface",
        "is",
        "literal_type",
        "namespace",
        "new",
        "number",
        "object",
        "of",
        "private",
        "private_property_identifier",
        "property_identifier",
        "protected",
        "public",
        "regex",
        "require",
        "return",
        "set",
        "static",
        "string",
        "super",
        "symbol",
        "this",
        "this_type",
        "throw",
        "type",
        "type_identifier",
        "typeof",
        "undefined",
        "var",
        "void",
        "while",
        "with",
        "yield",
    ])
});
/// "regular" [`Node::kind`]s that we print out as-is
static TS_REGULAR: LazyLock<HashSet<&str>> = LazyLock::new(|| {
    HashSet::from([
        "!", "!=", "!==", "=", "<", ">", "(", ")", "{", "}", "|", ";", ",", "=>", "===", ":", "?",
        ".", "\"", "${", "%", "%=", "&", "&&", "&&=", "&=", "'", "*", "**", "**=", "*=", "+", "++",
        "+=", "+?:", "-", "--", "-=", "-?:", ".", "...", "/", "/=", "<<", "<<=", "<=", "=", "==",
        ">=", ">>", ">>=", ">>>", ">>>=", "?.", "?:", "??", "??=", "@", "[", "]", "^", "^=", "`",
        "{|", "|=", "||", "||==", "|}", "~",
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
    fn highlight_node(&self, node: &Node, input: &[u8], prev_end: Option<Point>) -> Option<String> {
        let k = node.kind();
        let txt = node_text(node, input);
        if TS::is_special_node(k, txt) {
            let mut s = initial_padding(node, prev_end);
            write!(&mut s, "<code class=\"{}\">{}</code>", k, txt).expect("can't write");
            Some(s)
        } else if TS::is_regular_node(k) {
            let mut s = initial_padding(node, prev_end);
            s.push_str(txt);
            Some(s)
        } else {
            None
        }
    }
    /// always `true`
    fn should_highlight_children(&self, _node: &Node) -> bool {
        true
    }
}
