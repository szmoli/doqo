use std::collections::HashMap;

use common::{Documentation, LanguageProcessor, Symbol, SymbolTable, processor::{NodeHandler, ProcessingContext}};
use tree_sitter::Node;

use crate::handlers::{handle_struct_item};

pub struct RustProcessor {
  //fn(node: Node, source: &'static str, context: ProcessingContext)
  handlers: HashMap<&'static str, NodeHandler>,
}

impl RustProcessor {
  pub fn new() -> Self {
    let mut handlers: HashMap<&'static str, NodeHandler> = HashMap::new();

    handlers.insert("struct_item", handle_struct_item);

    Self {
      handlers: handlers
    }
  }

  pub fn register_handler(&mut self, kind: &'static str, handler: NodeHandler) -> Option<NodeHandler> {
    self.handlers.insert(kind, handler)
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
      
        //match node.kind() {
        //    "function_item" | "struct_item" | "mod_item" | "impl_item" | "trait_item" | "enum_item" | "const_item" | "field_declaration" => {
        //        let symbol = self.create_symbol(node, source, context);

        //        //println!("{:?}", symbol);
        //        //print_named_children(node, source);
        //        //println!();

        //        let symbol_name = symbol.name.clone();
        //        
        //        let symbol_id = context.register_symbol(symbol);

        //        context.namespace_stack.push(symbol_name);
        //        context.parent_id_stack.push(symbol_id);
        //        //context.symbols.push(symbol);

        //        true
        //    }
        //    //"doc_comment" => {
        //    "line_comment" | "block_comment" => {
        //        let text = source[node.byte_range()].trim();

        //        if text.starts_with("//!") || text.starts_with("/*!") {
        //            // INNER DOCS: Attach to the current active parent
        //            if let Some(&parent_id) = context.parent_id_stack.last() {
        //                context.symbol_table.attach_documentation(parent_id, Documentation::new(text.to_string()));
        //            }
        //        } else if text.starts_with("///") || text.starts_with("/**") {
        //            // OUTER DOCS: Buffer for the next sibling
        //            context.comment_buffer.push_str(text);
        //        }

        //        //context.comment_buffer.push_str(&source[node.byte_range()].to_string());

        //        false
        //    }
        //    _ => {
        //      //println!("comment_buffer:");
        //      //println!("{}", context.comment_buffer);
        //      //context.comment_buffer.clear();
        //      //println!("cleared comment_buffer");

        //      false
        //    }
        //}

        let handler = self.handlers.get(node.kind());
        
        match handler {
            Some(handler) => handler(node, source, context),
            None => false
        }
    }

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
}

