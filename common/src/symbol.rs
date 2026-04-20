use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use ts_rs::TS;

use crate::{Documentation};

/// The internal ID of a symbol.
/// 
/// Q: change to some kind of hash instead of usize?
pub type SymbolId = usize;

/// The symbol table. Maps IDs or FQIDs to Symbols.
/// The current_id starts from 0 and is incremented every time a new symbol is registered. 
#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SymbolTable {
  pub symbols: HashMap<SymbolId, Symbol>,
  fqid_index: HashMap<String, SymbolId>,
  #[serde(skip_serializing)]
  current_id: SymbolId,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            fqid_index: HashMap::new(),
            current_id: 0,
        }
    }

    /// Get the internal ID mapped to the FQID.
    pub fn symbol_id(&self, fqid: &str) -> Option<&SymbolId> {
        self.fqid_index.get(fqid)
    }

    /// Register a new symbol.
    /// Side effect: incements current_id by one.
    pub fn register_symbol(&mut self, symbol: Symbol) -> SymbolId {
        let id = self.current_id;
        self.fqid_index.insert(symbol.fqid().to_string(), id);
        self.symbols.insert(id, symbol);
        self.current_id += 1;
        id
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
#[ts(export)]
pub struct Symbol {
  /// Path of the scopes leading to the symbol.
  //scope: Vec<String>,

  /// Name of the symbol.
  //name: String, 
  kind: String,
  fqid: String,

  documentation: Documentation,

  pub parent: Option<SymbolId>,
  pub children: Vec<SymbolId>,
}

impl Symbol {
    pub fn new(name: &str, kind: &str, source: &str, scope: &[String], comments: &[String]) -> Self {
      Self {
        //scope: scope.clone(),
        //name: String::from(name),
        kind: String::from(kind),
        documentation: Documentation::new(source, comments),
        parent: None,
        children: Vec::new(),

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

    pub fn source(&self) -> &str {
      &self.documentation.source
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