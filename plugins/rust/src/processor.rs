use std::collections::HashMap;

use common::{Documentation, LanguageProcessor, Symbol, SymbolTable, processor::{NodeHandler, ProcessingContext}};
use tree_sitter::Node;

use crate::handlers::{handle_enum_item, handle_enum_variant, handle_function_item, handle_line_comment, handle_mod_item, handle_struct_item, handle_trait_item};

pub struct RustProcessor {
  //fn(node: Node, source: &'static str, context: ProcessingContext)
  handlers: HashMap<&'static str, NodeHandler>,
}

impl RustProcessor {
  pub fn new() -> Self {
    let mut handlers: HashMap<&'static str, NodeHandler> = HashMap::new();

    handlers.insert("struct_item", handle_struct_item);
    handlers.insert("enum_item", handle_enum_item);
    handlers.insert("enum_variant", handle_enum_variant);
    handlers.insert("function_item", handle_function_item);
    handlers.insert("trait_item", handle_trait_item);
    handlers.insert("mod_item", handle_mod_item);
    handlers.insert("line_comment", handle_line_comment);

    Self {
      handlers: handlers
    }
  }
}

impl LanguageProcessor for RustProcessor {
    fn language(&self) -> tree_sitter::Language {
        tree_sitter_rust::LANGUAGE.into()
    }

    /// Returns true if stack was pushed. Returns false otherwise.
    fn handle_node(
        &self,
        node: tree_sitter::Node,
        source: &str,
        context: &mut common::processor::ProcessingContext,
    ) -> bool {
        if let Some(handler) = self.handlers.get(node.kind()) {
          handler(node, source, context)
        }
        else {
          if node.is_named() {
            // TODO: clear comment buffer
          }

          false
        }
    }

    /* 
    fn create_symbol(
        &self,
        node: tree_sitter::Node,
        source: &str,
        context: &mut common::processor::ProcessingContext,
    ) -> common::Symbol {
        let symbol_source = &source[node.byte_range()];
        let symbol_name = self.get_symbol_name(node, source);
        let symbol_kind = node.kind().to_string();
        let symbol_namespace = context.namespace_stack.clone();

        // TODO: work out documentation logic.
        /*
        let symbol_documentation = if !context.comment_buffer.is_empty() {
          let content = context.comment_buffer.clone();
          context.comment_buffer.clear();
          Some(Documentation::new(content))
        } 
        else {
          None
        };
        */

        Symbol::new(symbol_name, symbol_kind, symbol_namespace, symbol_source, None)
    }
    */

    /*
    fn get_symbol_name(&self, node: Node, source: &str) -> String {
        let identifier_node = node.child_by_field_name("name");
        let trait_node = node.child_by_field_name("trait");
        let type_node = node.child_by_field_name("type");

        //println!("{:?}, {:?}, {:?}", identifier_node, trait_node, type_node);

        match (identifier_node, trait_node, type_node) {
            (Some(identifier_node), None, None) => {
              source[identifier_node.byte_range()].to_string()
            }
            (None, Some(trait_node), None) => {
              source[trait_node.byte_range()].to_string()
            }
            (None, None, Some(type_node)) => {
              source[type_node.byte_range()].to_string()
            }
            _ => {
              format!("anonymous_{}", node.kind())
            }
        }
    }
    */
}

