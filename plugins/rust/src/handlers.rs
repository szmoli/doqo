use common::{Documentation, Symbol, processor::ProcessingContext};
use tree_sitter::Node;

// Function template:
// pub fn handle(node: Node, source: &str, context: &mut ProcessingContext) -> bool;

fn get_symbol_name(node: Node, source: &str, field_name: &str) -> String {
  node.child_by_field_name(field_name)
    .map(|n| source[n.byte_range()].to_string())
    .unwrap_or_else(|| format!("anonymous_{}", node.kind()).to_string())
}

// Core data, logic

pub fn handle_struct_item(node: Node, source: &str, context: &mut ProcessingContext) -> bool {
  let name = get_symbol_name(node, source, "name");

  //println!("Handled struct: {}", name);
  //println!("\tNode: {:?}", node);

  let documentation = context.make_documentation();

  //println!("\tDocumentation: {:?}", documentation);

  let symbol = Symbol::new(
    name.clone(), 
    node.kind().to_string(), 
    context.namespace(), 
    &source[node.byte_range()], 
    documentation
  );

  let id = context.register_symbol(symbol);

  //println!("\tID: {}", id);

  context.push(id, name.clone());
  true
}

pub fn handle_enum_item(node: Node, source: &str, context: &mut ProcessingContext) -> bool {
  let name = get_symbol_name(node, source, "name");
  let documentation = context.make_documentation();
  let symbol = Symbol::new(
    name.clone(),
    node.kind().to_string(), 
    context.namespace(), 
    &source[node.byte_range()], 
    documentation
  );
  let id = context.register_symbol(symbol);

  context.push(id, name.clone());
  true
}

pub fn handle_enum_variant(node: Node, source: &str, context: &mut ProcessingContext) -> bool {
  let name = get_symbol_name(node, source, "name");
  let documentation = context.make_documentation();
  let symbol = Symbol::new(
    name.clone(),
    node.kind().to_string(),
    context.namespace(),
    &source[node.byte_range()],
    documentation
  );
  let id = context.register_symbol(symbol);

  context.push(id, name);
  true
}

pub fn handle_function_item(node: Node, source: &str, context: &mut ProcessingContext) -> bool {
  let name = get_symbol_name(node, source, "name");
  let documentation = context.make_documentation();
  let symbol = Symbol::new(
    name.clone(), 
    node.kind().to_string(), 
    context.namespace(), 
    &source[node.byte_range()], 
    documentation
  );
  let id = context.register_symbol(symbol);

  context.push(id, name);
  true
}

// Traits and implementations

pub fn handle_trait_item(node: Node, source: &str, context: &mut ProcessingContext) -> bool {
  let name = get_symbol_name(node, source, "name");
  let documentation = context.make_documentation();
  let symbol = Symbol::new(
    name.clone(), 
    node.kind().to_string(), 
    context.namespace(), 
    &source[node.byte_range()], 
    documentation
  );
  let id = context.register_symbol(symbol);

  context.push(id, name);
  true
}

pub fn handle_impl_item(node: Node, source: &str, context: &mut ProcessingContext) -> bool {
  // TODO
  false
}

// Containers for inner docs

pub fn handle_mod_item(node: Node, source: &str, context: &mut ProcessingContext) -> bool {
  let name = get_symbol_name(node, source, "name");
  let documentation = context.make_documentation();
  let symbol = Symbol::new(
    name.clone(), 
    node.kind().to_string(), 
    context.namespace(), 
    &source[node.byte_range()], 
    documentation
  );
  let id = context.register_symbol(symbol);

  context.push(id, name);
  true
}

pub fn handle_source_file(node: Node, source: &str, context: &mut ProcessingContext) -> bool {
  // TODO
  false
}

// Comments

pub fn handle_line_comment(node: Node, source: &str, context: &mut ProcessingContext) -> bool {
  if let Some(_outer_marker) = node.child_by_field_name("outer") {
    context.append_comment(&source[node.byte_range()]);
  }

  if let Some(_inner_marker) = node.child_by_field_name("inner") {
    if let Some(content) = node.child_by_field_name("doc").map(|n| &source[n.byte_range()]) {
      // TODO: append to parent
    }
  }

  false
}

pub fn handle_block_comment(node: Node, source: &str, context: &mut ProcessingContext) -> bool {
  // TODO
  false
}
