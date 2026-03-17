use common::{Documentation, LanguagePlugin, LanguageProcessor, Symbol};
use common::utility::{print_named_children};
use std::collections::HashSet;
use tree_sitter::Node;
use tree_sitter_rust;

pub struct RustPlugin;
pub struct RustProcessor;

impl LanguageProcessor for RustProcessor {
    fn language(&self) -> tree_sitter::Language {
        tree_sitter_rust::LANGUAGE.into()
    }

    fn handle_node(
        &self,
        node: tree_sitter::Node,
        source: &str,
        context: &mut common::processor::ProcessingContext,
    ) -> bool {
        match node.kind() {
            "function_item" | "struct_item" | "mod_item" | "impl_item" | "trait_item" | "enum_item" => {
                let symbol = self.create_symbol(node, source, context);

                println!("{:?}", symbol);
                print_named_children(node, source);
                println!();

                let symbol_name = symbol.name.clone();
                let symbol_id = context.register_symbol(symbol);

                context.namespace_stack.push(symbol_name);
                context.parent_id_stack.push(symbol_id);
                //context.symbols.push(symbol);

                true
            }
            "line_comment" | "block_comment" => {
                context.comment_buffer.push_str(&source[node.byte_range()].to_string());

                false
            }
            _ => {
              false
            }
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

        println!("{:?}, {:?}, {:?}", identifier_node, trait_node, type_node);

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
        HashSet::from(["struct_item"])
        //kinds.iter().map)
    }
}
