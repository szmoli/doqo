use std::{collections::HashSet};
use common::{LanguagePlugin, LanguageProcessor, Symbol};
use tree_sitter_rust;

pub struct RustPlugin;
pub struct RustProcessor;

impl LanguageProcessor for RustProcessor {
    fn language(&self) -> tree_sitter::Language {
        tree_sitter_rust::LANGUAGE.into()
    }

    fn handle_node(&self, node: tree_sitter::Node, source: &str, context: &mut common::processor::ProcessingContext) -> bool {
      match node.kind() {
        "function_item" | "struct_item" | "mod_item" => {
            let symbol = self.create_symbol(node, source, context);
            println!("{:?}", symbol);
            context.symbols.push(symbol);
            true 
        }
        "line_comment" | "block_comment" => {
            context.comment_buffer += &source[node.byte_range()].to_string();
            false
        }
        _ => false 
      }
    }
    
    fn create_symbol(&self, node: tree_sitter::Node, source: &str, context: &common::processor::ProcessingContext) -> common::Symbol {
        //todo!()
        let node_source = &source[node.byte_range()];
        let node_name = format!("node{}", context.namespace_stack.len());
        Symbol::new(node_name, node_source)
    }
    
    /*
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
    */
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