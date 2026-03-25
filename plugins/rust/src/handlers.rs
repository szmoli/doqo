use common::processor::ProcessingContext;
use tree_sitter::Node;

pub fn handle_struct_item(node: Node, source: &str, context: &mut ProcessingContext) -> bool {
  println!("{:?}", node);
  true
}