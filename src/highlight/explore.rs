use crate::highlight::Highlight;
use crate::highlight::shared::node_text;
use std::collections::HashSet;
use tree_sitter::{Language, Node, Point};

pub struct Explore {
    l: Language,
    special: HashSet<String>,
    regular: HashSet<String>,
}

impl Explore {
    pub fn new<I: Into<HashSet<String>>>(l: Language, special: I, regular: I) -> Self {
        Self {
            l,
            special: special.into(),
            regular: regular.into(),
        }
    }
    pub fn new_ts<I: Into<HashSet<String>>>(special: I, regular: I) -> Self {
        Self::new(
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            special,
            regular,
        )
    }
    pub fn new_rust<I: Into<HashSet<String>>>(special: I, regular: I) -> Self {
        Self::new(tree_sitter_rust::LANGUAGE.into(), special, regular)
    }
}

impl Highlight for Explore {
    fn language(&self) -> Language {
        self.l.clone()
    }
    fn highlight_node(&self, node: &Node, input: &[u8], _prev_end: Option<Point>) -> String {
        let k = node.kind();
        let txt = node_text(node, input);
        if self.special.contains(k) {
            format!("special node {:?} {}\n", node, txt)
        } else if self.regular.contains(k) {
            format!("regular node {:?} {}\n", node, txt)
        } else {
            String::new()
        }
    }
}

pub struct ExploreAll {
    l: Language,
}

impl ExploreAll {
    pub fn new(l: Language) -> Self {
        Self { l }
    }
    pub fn new_ts() -> Self {
        Self::new(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
    }
    pub fn new_rust() -> Self {
        Self::new(tree_sitter_rust::LANGUAGE.into())
    }
}

impl Highlight for ExploreAll {
    fn language(&self) -> Language {
        self.l.clone()
    }
    fn highlight_node(&self, node: &Node, input: &[u8], _prev_end: Option<Point>) -> String {
        let txt = node_text(node, input);
        format!("{:?} {}\n", node, txt)
    }
}
