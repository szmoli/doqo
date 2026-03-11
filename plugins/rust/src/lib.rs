use std::collections::HashSet;
use common::{LanguagePlugin, LanguageProcessor, Symbol};
use tree_sitter_rust;
use tree_sitter::{Parser};

pub struct RustPlugin;
pub struct RustProcessor;

impl LanguageProcessor for RustProcessor {
    fn language(&self) -> tree_sitter::Language {
        tree_sitter_rust::LANGUAGE.into()
    }

    fn extract_symbols(&self, source: &str) -> Vec<Symbol> {
        let mut parser = Parser::new();
        parser.set_language(&self.language()).expect("Failed to set parser language.");

        let tree = parser.parse(source, None).expect("Failed to parse tree.");
        let symbols = Vec::new();
        let mut cursor = tree.walk();
        
        if cursor.goto_first_child() {
            loop {
                let node = cursor.node();
                println!("{:?}", node);
                
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }

        symbols
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
}