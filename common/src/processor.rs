use std::{io, task::Context};

use crate::{Documentation, Symbol, SymbolId, SymbolTable, symbol};
use tree_sitter::{Node, Parser};

pub type NodeHandler = fn(node: Node, source: &str, &mut ProcessingContext) -> bool;

//pub struct ProcessingContext<'a> {
pub struct ProcessingContext<'a> {
  namespace_stack: Vec<String>,
  //pub symbols: Vec<Symbol>,
  comment_buffer: String,
  symbol_table: &'a mut SymbolTable,
  parent_id_stack: Vec<SymbolId>,
}

impl<'a> ProcessingContext<'a> {
//impl ProcessingContext {
  pub fn new(symbol_table: &'a mut SymbolTable) -> Self {
  //pub fn new() -> Self {
    Self {
      namespace_stack: Vec::new(),
      comment_buffer: String::new(),
      symbol_table: symbol_table,
      parent_id_stack: Vec::new(),
    }
  }

  /// Makes a new Documentation from the comment buffer.
  /// Side effect: clears the comment buffer.
  pub fn make_documentation(&mut self) -> Option<Documentation> {
    if !self.comment_buffer.is_empty() {
      let raw = self.comment_buffer.clone();
      self.comment_buffer.clear();
      Some(Documentation::new(raw))
    }
    else {
      None
    }
  }

  pub fn namespace(&self) -> Vec<String> {
    self.namespace_stack.clone()
  }

  pub fn push(&mut self, id: SymbolId, name: String) {
    self.namespace_stack.push(name);
    self.parent_id_stack.push(id);
    debug_assert_eq!(self.parent_id_stack.len(), self.namespace_stack.len());
  }

  pub fn pop(&mut self) -> Option<(SymbolId, String)> {
    let result = self.parent_id_stack.pop().zip(self.namespace_stack.pop());
    debug_assert_eq!(self.parent_id_stack.len(), self.namespace_stack.len());
    result
  }

  pub fn register_symbol(&mut self, mut symbol: Symbol) -> SymbolId {
    let parent_id = self.parent_id_stack.last().copied();
    symbol.parent = parent_id;

    let id = self.symbol_table.register_symbol(symbol);

    if let Some(parent_id) = parent_id {
      self.symbol_table.link_child(parent_id, id);
    }

    id
  }

  pub fn append_comment(&mut self, text: &str) {
    self.comment_buffer.push_str(text);
  }

  /*
  pub fn register_symbol(&mut self, mut symbol: Symbol) -> SymbolId {
    symbol.parent = self.parent_id_stack.last().copied();

    // TODO: work out comment attaching logic
    
    if !self.comment_buffer.is_empty() {
      symbol.documentation = Some(Documentation::new(self.comment_buffer.clone()));
      self.comment_buffer.clear();
    }
    

    let id = self.symbol_table.register_symbol(symbol);

    if let Some(&parent_id) = self.parent_id_stack.last() {
      self.symbol_table.link_child(parent_id, id);
    }

    id
  }
  */
}

/// Processes a specific language into symbols.
pub trait LanguageProcessor {
    /// Get the Tree Sitter grammar for the language.
    fn language(&self) -> tree_sitter::Language;

    /// Extract the symbols from a source string.
    fn process(&self, source: &str, symbol_table: &mut SymbolTable) {
        let mut context = ProcessingContext::new(symbol_table);

        let mut parser = Parser::new();
        parser
            .set_language(&self.language())
            .expect("Failed to set parser language.");

        let tree = parser.parse(source, None).expect("Failed to parse tree.");
        self.walk_recursive(tree.root_node(), source, &mut context);
    }

    fn walk_recursive(&self, node: Node, source: &str, context: &mut ProcessingContext) {
      //let mut input = String::new();
      //io::stdin().read_line(&mut input).unwrap();
      //println!("{:?}", node);

      let pushed_stack = self.handle_node(node, source, context);

      let mut cursor = node.walk();
      for child in node.named_children(&mut cursor) {
        self.walk_recursive(child, source, context);
      }

      if pushed_stack {
        context.pop();
      }
    }

    fn handle_node(&self, node: Node, source: &str, context: &mut ProcessingContext) -> bool;

    //fn create_symbol(&self, node: Node, source: &str, context: &mut ProcessingContext) -> Symbol;

    //fn get_symbol_name(&self, node: Node, source: &str) -> String;
}
