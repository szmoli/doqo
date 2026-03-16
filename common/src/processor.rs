use crate::Symbol;
use tree_sitter::{Node, Parser};

pub struct ProcessingContext {
  pub namespace_stack: Vec<String>,
  pub symbols: Vec<Symbol>,
  pub comment_buffer: String,
}

impl ProcessingContext {
  pub fn new() -> Self {
    Self {
      namespace_stack: Vec::new(),
      symbols: Vec::new(),
      comment_buffer: String::new(),
    }
  }
}

/// Processes a specific language into symbols.
pub trait LanguageProcessor {
    /// Get the Tree Sitter grammar for the language.
    fn language(&self) -> tree_sitter::Language;

    /// Extract the symbols from a source string.
    //fn extract_symbols(&self, source: &str) -> Vec<Symbol>;
    fn process(&self, source: &str) -> Vec<Symbol> {
        let mut context = ProcessingContext::new();
        let mut parser = Parser::new();
        parser
            .set_language(&self.language())
            .expect("Failed to set parser language.");

        let tree = parser.parse(source, None).expect("Failed to parse tree.");
        let symbols = Vec::new();
        //let mut cursor = tree.walk();

        self.walk_recursive(tree.root_node(), source, &mut context);

        /*
        if cursor.goto_first_child() {
            loop {
                let node = cursor.node();
                let node_text = &source[node.byte_range()];

                println!("{:?}", node);
                println!("Child count: {}", node.child_count());
                println!("Named child count: {}", node.named_child_count());
                println!("Children:");
                for i in 0..node.child_count() {
                    println!("\t{:?}", node.child(i.try_into().unwrap()));
                }
                println!("{}", node_text);
                println!();

                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
        */

        symbols
    }

    fn walk_recursive(&self, node: Node, source: &str, context: &mut ProcessingContext) {
      let pushed_stack = self.handle_node(node, source, context);

      let mut cursor = node.walk();
      for child in node.named_children(&mut cursor) {
        self.walk_recursive(child, source, context);
      }

      if pushed_stack {
        context.namespace_stack.pop();
      }
    }

    fn handle_node(&self, node: Node, source: &str, context: &mut ProcessingContext) -> bool;

    fn create_symbol(&self, node: Node, source: &str, context: &ProcessingContext) -> Symbol;
}
