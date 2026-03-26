use std::collections::HashMap;

use common::{Documentation, LanguageProcessor, Symbol, SymbolTable, processor::{NodeHandler, ProcessingContext}};
use tree_sitter::Node;

use crate::handlers::{handle_enum_item, handle_enum_variant, handle_function_item, handle_line_comment, handle_mod_item, handle_struct_item, handle_trait_item};

pub struct RustProcessor {
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
}

