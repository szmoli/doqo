use std::{collections::HashMap, fs::{self, File}, path::{Path, PathBuf}};
use serde::{Serialize, Deserialize};
use ts_rs::TS;

use crate::{Documentation, source::{FileId, SourceFile}};

#[derive(Debug, Serialize, Deserialize, TS)]
pub struct Span {
  pub file_id: FileId,
  pub start: usize,
  pub end: usize,
}

/// The internal ID of a symbol.
/// 
/// Q: change to some kind of hash instead of usize?
pub type SymbolId = usize;

/// The symbol table. Maps IDs or FQIDs to Symbols.
/// The current_id starts from 0 and is incremented every time a new symbol is registered. 
#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, rename = "DoqoSymbolTable")]
pub struct SymbolTable {
  pub symbols: HashMap<SymbolId, Symbol>,
  pub sources: HashMap<FileId, SourceFile>,
  fqid_index: HashMap<String, SymbolId>,
  #[serde(skip_serializing)]
  #[ts(skip)]
  current_symbol_id: SymbolId,
  #[serde(skip_serializing)]
  #[ts(skip)]
  current_file_id: FileId,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            fqid_index: HashMap::new(),
            sources: HashMap::new(),
            current_symbol_id: 0,
            current_file_id: 0,
        }
    }

    /// Get the internal ID mapped to the FQID.
    pub fn symbol_id(&self, fqid: &str) -> Option<&SymbolId> {
        self.fqid_index.get(fqid)
    }

    /// Register a new symbol.
    /// Side effect: incements current_id by one.
    pub fn register_symbol(&mut self, symbol: Symbol) -> SymbolId {
        let id = self.current_symbol_id;
        self.fqid_index.insert(symbol.fqid().to_string(), id);
        self.symbols.insert(id, symbol);
        self.current_symbol_id += 1;
        id
    }

    pub fn register_file(&mut self, path: PathBuf) -> FileId {
      let id = self.current_file_id;

      let content = fs::read_to_string(&path).expect("Couldn't read file.");
      let source_file = SourceFile { path, content };
      self.sources.insert(id, source_file);

      self.current_file_id += 1;
      id
    }

    pub fn source(&self, file_id: &FileId) -> String {
      self.sources.get(&file_id).expect("File not found.").content.clone()
    }

    pub fn link_child(&mut self, parent_id: SymbolId, child_id: SymbolId) {
      if let Some(parent) = self.symbols.get_mut(&parent_id) {
        if !parent.children.contains(&child_id) {
          parent.children.push(child_id);
        }
      }
    }

    /// Get the Symbol mapped to the FQID.
    //pub fn find_by_fqid(&self, fqid: &String) -> Option<&Symbol> {
    //    let id = self.fqid_index.get(fqid).expect("FQID {fqid} not found");
    //    self.symbols.get(id)
    //}

    /// Get the Symbol mapped to the internal ID.
    pub fn get(&self, id: &SymbolId) -> Option<&Symbol> {
        self.symbols.get(id)
    }

    pub fn get_mut(&mut self, id: &SymbolId) -> Option<&mut Symbol> {
      self.symbols.get_mut(id)
    }

    pub fn append_comment(&mut self, id: &SymbolId, comment: &str) {
      if let Some(symbol) = self.symbols.get_mut(id) {
        symbol.append_comment(comment);
      }
    }

    pub fn json(&self) -> String {
      //serde_json::to_string(self).unwrap()
      serde_json::to_string_pretty(self).unwrap()
    }
}

/// Holds information about a single symbol in the source.
#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, rename = "DoqoSymbol")]
pub struct Symbol {
  /// Path of the scopes leading to the symbol.
  //scope: Vec<String>,

  /// Name of the symbol.
  //name: String, 
  kind: String,
  fqid: String,

  span: Span,
  documentation: Documentation,

  pub parent: Option<SymbolId>,
  pub children: Vec<SymbolId>,
}

impl Symbol {
    pub fn new(name: &str, kind: &str, file_id: FileId, start: usize, end: usize, scope: &[String], comments: &[String]) -> Self {
      Self {
        //scope: scope.clone(),
        //name: String::from(name),
        kind: String::from(kind),
        documentation: Documentation::new(comments),
        parent: None,
        children: Vec::new(),

        span: Span { file_id, start, end },

        fqid: if scope.is_empty() {
          String::from(name)
        } else {
          format!("{}::{}", scope.join("::"), name)
        }
      }
    }

    pub fn name(&self) -> &str {
      self.fqid.rsplit_once("::").map(|(_scope, name)| name).unwrap_or(self.fqid.as_str())
    }

    pub fn scope(&self) -> &str {
      self.fqid.rsplit_once("::").map(|(scope, _name)| scope).unwrap_or("")
    }

    pub fn span(&self) -> &Span {
      &self.span
    }

    pub fn comments(&self) -> String {
      self.documentation.comments()
    }

    pub fn fqid(&self) -> &str {
      &self.fqid
    }

    pub fn append_comment(&mut self, comment: &str) {
      self.documentation.append(comment);
    }
}