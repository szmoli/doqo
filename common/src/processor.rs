use std::{ffi::OsStr, fs, mem::take, path::{Path, PathBuf}};

use crate::{Symbol, SymbolId, SymbolTable, source::{self, FileId, SourceFile}};
use tree_sitter::{Node, Parser};

pub type NodeHandler = fn(node: Node, source: &str, &mut ProcessingContext) -> bool;

//pub struct ProcessingContext<'a> {
pub struct ProcessingContext<'a> {
  scope: Vec<String>,
  //pub symbols: Vec<Symbol>,
  comment_buffer: Vec<String>,
  symbol_table: &'a mut SymbolTable,
  id_scope: Vec<SymbolId>,
  file_id: FileId,
}

impl<'a> ProcessingContext<'a> {
//impl ProcessingContext {
  pub fn new(symbol_table: &'a mut SymbolTable, file_id: FileId) -> Self {
  //pub fn new() -> Self {
    Self {
      scope: Vec::new(),
      comment_buffer: Vec::new(),
      symbol_table: symbol_table,
      id_scope: Vec::new(),
      file_id: file_id,
    }
  }

  pub fn file_id(&self) -> &FileId {
    &self.file_id
  }

  pub fn file(&self) -> &SourceFile {
    &self.symbol_table.sources.get(&self.file_id).expect("File not registered")
  }

  /// Makes a new Documentation from the comment buffer.
  /// 
  /// Side effect: clears the comment buffer.
  pub fn take_comments(&mut self) -> Vec<String> {
    take(&mut self.comment_buffer)
  }

  pub fn scope(&self) -> &Vec<String> {
    &self.scope
  }

  pub fn push(&mut self, id: SymbolId, name: &str) {
    self.scope.push(String::from(name));
    self.id_scope.push(id);
    debug_assert_eq!(self.id_scope.len(), self.scope.len());
  }

  pub fn pop(&mut self) -> Option<(SymbolId, String)> {
    let result = self.id_scope.pop().zip(self.scope.pop());
    debug_assert_eq!(self.id_scope.len(), self.scope.len());
    result
  }

  pub fn top(&self) -> Option<&Symbol> {
    let id = self.id_scope.last()?;
    self.symbol_table.get(id)
  }

  pub fn top_mut(&mut self) -> Option<&mut Symbol> {
    let id = self.id_scope.last()?;
    self.symbol_table.get_mut(id)
  }

  pub fn register_symbol(&mut self, mut symbol: Symbol) -> SymbolId {
    let parent_id = self.id_scope.last().copied();
    symbol.parent = parent_id;

    let id = self.symbol_table.register_symbol(symbol);

    if let Some(parent_id) = parent_id {
      self.symbol_table.link_child(parent_id, id);
    }

    id
  }

  pub fn push_comment(&mut self, text: &str) {
    self.comment_buffer.push(String::from(text));
  }
}

/// Processes a specific language into symbols.
pub trait LanguageProcessor {
    /// Get the Tree Sitter grammar for the language.
    fn language(&self) -> tree_sitter::Language;

    /// Extract the symbols from a source string.
    fn process(&self, source_path: &PathBuf, symbol_table: &mut SymbolTable) {
        //let source = fs::read_to_string(&source_path).expect("Failed to read input");
        //let filename = source_path.file_name().and_then(|f| f.to_str()).unwrap_or("unknown_file");
        let file_id = symbol_table.register_file(source_path.to_path_buf());
        let source = symbol_table.source(&file_id);

        let mut context = ProcessingContext::new(symbol_table, file_id);

        let mut parser = Parser::new();
        parser
            .set_language(&self.language())
            .expect("Failed to set parser language.");

        let tree = parser.parse(source.as_str(), None).expect("Failed to parse tree.");
        self.walk_recursive(tree.root_node(), source.as_str(), &mut context);
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
