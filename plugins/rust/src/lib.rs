use std::{collections::HashSet};
use common::{LanguagePlugin, LanguageProcessor};
use tree_sitter_rust;

pub struct RustPlugin;
pub struct RustProcessor;

impl LanguageProcessor for RustProcessor {
    fn language(&self) -> tree_sitter::Language {
        tree_sitter_rust::LANGUAGE.into()
    }
    
    fn is_symbol(&self, node: tree_sitter::Node) -> bool {
        todo!()
    }
    
    fn node_name(&self, node: tree_sitter::Node, source: &str) -> String {
        todo!()
    }
    
    fn is_comment(&self, node: tree_sitter::Node) -> bool {
        todo!()
    }
    
    fn sticks_to(&self, node: tree_sitter::Node, source: &str) -> common::StickLocation {
        todo!()
    }
}

impl LanguagePlugin for RustPlugin {
    fn name(&self) -> &'static str {
        "Rust"
    }

    fn extensions(&self) -> HashSet<&'static str> {
        HashSet::from(["rs"])
    }

    fn processor(&self) -> Box<dyn LanguageProcessor> {
        Box::new(RustProcessor)
    }
    
    fn id(&self) -> &'static str {
        "rust"
    }
    
    fn symbol_kinds(&self) -> HashSet<&'static str> {
        // TODO
        HashSet::from(["struct"])
        //kinds.iter().map)
    }
}