use std::ffi::OsStr;

use common::{Symbol, SymbolId, processor::ProcessingContext};
use tree_sitter::Node;

// Function template:
// pub fn handle(node: Node, source: &str, context: &mut ProcessingContext) -> bool;

fn node_name(node: Node, source: &str, name_field: &str) -> String {
  node.child_by_field_name(name_field)
    .map(|n| source[n.byte_range()].to_string())
    .unwrap_or_else(|| format!("anonymous_{}", node.kind()))
}

fn create_symbol(node: Node, source_id: SymbolId, from: usize, to: usize, name: &str, context: &mut ProcessingContext) -> Symbol {
  let comments = context.take_comments();
  
  Symbol::new(&name, node.kind(), source_id, from, to, context.scope(), &comments)
}

// Core data, logic

pub fn handle_type_item(node: Node, source: &str, context: &mut ProcessingContext) -> bool {
  let name = node_name(node, source, "name");
  let symbol = create_symbol(node, context.file_id().to_owned(), node.start_byte(), node.end_byte(), &name, context);

  let name = symbol.name().to_string();
  let id = context.register_symbol(symbol);

  context.push(id, &name);
  true
}

pub fn handle_field_declaration(node: Node, source: &str, context: &mut ProcessingContext) -> bool {
  let name = node_name(node, source, "name");
  let symbol = create_symbol(node, context.file_id().to_owned(), node.start_byte(), node.end_byte(), &name, context);

  let _id = context.register_symbol(symbol);

  false
}

pub fn handle_macro_definition(node: Node, source: &str, context: &mut ProcessingContext) -> bool {
  let name = node_name(node, source, "name");
  let symbol = create_symbol(node, context.file_id().to_owned(), node.start_byte(), node.end_byte(), &name, context);

  let _id = context.register_symbol(symbol);

  false
}

pub fn handle_const_item(node: Node, source: &str, context: &mut ProcessingContext) -> bool {
  let name = node_name(node, source, "name");
  let symbol = create_symbol(node, context.file_id().to_owned(), node.start_byte(), node.end_byte(), &name, context);

  let _id = context.register_symbol(symbol);

  false
}

pub fn handle_struct_item(node: Node, source: &str, context: &mut ProcessingContext) -> bool {
  let name = node_name(node, source, "name");
  let symbol = create_symbol(node, context.file_id().to_owned(), node.start_byte(), node.end_byte(), &name, context);

  let name = symbol.name().to_string();
  let id = context.register_symbol(symbol);

  context.push(id, &name);
  true
}

pub fn handle_enum_item(node: Node, source: &str, context: &mut ProcessingContext) -> bool {
  let name = node_name(node, source, "name");
  let symbol = create_symbol(node, context.file_id().to_owned(), node.start_byte(), node.end_byte(), &name, context);

  let name = symbol.name().to_string();
  let id = context.register_symbol(symbol);

  context.push(id, &name);
  true
}

pub fn handle_enum_variant(node: Node, source: &str, context: &mut ProcessingContext) -> bool {
  let name = node_name(node, source, "name");
  let symbol = create_symbol(node, context.file_id().to_owned(), node.start_byte(), node.end_byte(), &name, context);

  let name = symbol.name().to_string();
  let id = context.register_symbol(symbol);

  context.push(id, &name);
  true
}

pub fn handle_function_item(node: Node, source: &str, context: &mut ProcessingContext) -> bool {
  let name = node_name(node, source, "name");
  let symbol = create_symbol(node, context.file_id().to_owned(), node.start_byte(), node.end_byte(), &name, context);

  let name = symbol.name().to_string();
  let id = context.register_symbol(symbol);

  context.push(id, &name);
  true
}

// Traits and implementations

pub fn handle_trait_item(node: Node, source: &str, context: &mut ProcessingContext) -> bool {
  let name = node_name(node, source, "name");
  let symbol = create_symbol(node, context.file_id().to_owned(), node.start_byte(), node.end_byte(), &name, context);

  let name = symbol.name().to_string();
  let id = context.register_symbol(symbol);

  context.push(id, &name);
  true
}

pub fn handle_impl_item(node: Node, source: &str, context: &mut ProcessingContext) -> bool {
  // TODO
  false
}

// Containers for inner docs

pub fn handle_mod_item(node: Node, source: &str, context: &mut ProcessingContext) -> bool {
  let name = node_name(node, source, "name");
  let symbol = create_symbol(node, context.file_id().to_owned(), node.start_byte(), node.end_byte(), &name, context);

  let id = context.register_symbol(symbol);

  context.push(id, &name);
  true
}

pub fn handle_source_file(node: Node, source: &str, context: &mut ProcessingContext) -> bool {
  let name = context.file().path.file_name().expect("No filename").to_string_lossy().into_owned();
  let symbol = create_symbol(node, context.file_id().to_owned(), node.start_byte(), node.end_byte(), &name, context);

  let id = context.register_symbol(symbol);
  
  context.push(id, &name);
  true
}

// Comments

pub fn handle_line_comment(node: Node, source: &str, context: &mut ProcessingContext) -> bool {
  if let Some(_outer_marker) = node.child_by_field_name("outer") {
    if let Some(content) = node.child_by_field_name("doc").map(|n| &source[n.byte_range()]) {
      context.push_comment(content.trim());
    }
  }

  if let Some(_inner_marker) = node.child_by_field_name("inner") {
    if let Some(content) = node.child_by_field_name("doc").map(|n| &source[n.byte_range()]) {
      if let Some(symbol) = context.top_mut() {
        symbol.append_comment(content.trim());
      }
    }
  }

  false
}

pub fn handle_block_comment(node: Node, source: &str, context: &mut ProcessingContext) -> bool {
  // TODO
  false
}


