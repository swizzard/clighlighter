//! # explore
//!
//! these structs' [`Highlight`] implementations print information about [`tree_sitter::Node`]s and
//! can be used while developing more useful highlighters
use crate::highlight::Highlight;
use crate::highlight::shared::node_text;
use std::collections::HashSet;
use tree_sitter::{Language, Node, Point};

/// a struct that implements [`Highlight`] to print information about [`Node`]s based on their
/// [`Node::kind`]
///
/// I found that [`Node::kind`]s can be broadly sorted into three buckets:    
///   - kinds you want to highlight specifically
///   - kinds you want to print out with no highlighting
///   - kinds you want to ignore    
///
/// This struct lets you experiment with those buckets
pub struct Explore {
    l: Language,
    /// node kinds you plan to highlight
    special: HashSet<String>,
    /// node kinds you plan to print unadorned
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
    /// prints node debug info, as well as whether the node is `special` or `regular`
    fn highlight_node(
        &self,
        node: &Node,
        input: &[u8],
        _prev_end: Option<Point>,
    ) -> Option<String> {
        let k = node.kind();
        let txt = node_text(node, input);
        if self.special.contains(k) {
            Some(format!("special node {:?} {}\n", node, txt))
        } else if self.regular.contains(k) {
            Some(format!("regular node {:?} {}\n", node, txt))
        } else {
            None
        }
    }
    /// always `true` because we want to examine every node
    fn should_highlight_children(&self, _node: &Node) -> bool {
        true
    }
}

/// a struct that implements [`Highlight`] to print information about every [`Node`]
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
    /// prints the same debug info for every node
    fn highlight_node(
        &self,
        node: &Node,
        input: &[u8],
        _prev_end: Option<Point>,
    ) -> Option<String> {
        let txt = node_text(node, input);
        Some(format!("{:?} {}\n", node, txt))
    }
    /// always `true` because we want to examine every node
    fn should_highlight_children(&self, _node: &Node) -> bool {
        true
    }
}
