use std::collections::HashMap;

use crate::Documentation;

/// The internal ID of a symbol.
/// 
/// Q: change to some kind of hash instead of usize?
pub type SymbolId = usize;

/// The symbol table. Maps IDs or FQIDs to Symbols.
/// The current_id starts from 0 and is incremented every time a new symbol is registered. 
pub struct SymbolTable {
  pub symbols: HashMap<SymbolId, Symbol>,
  fqid_index: HashMap<String, SymbolId>,
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
    pub fn symbol_id(&self, fqid: &String) -> Option<&SymbolId> {
        self.fqid_index.get(fqid)
    }

    /// Register a new symbol.
    /// Side effect: incements current_id by one.
    pub fn register_symbol(&mut self, symbol: Symbol) -> SymbolId {
        let id = self.current_id;
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
    pub fn find_by_fqid(&self, fqid: &String) -> Option<&Symbol> {
        let id = self.fqid_index.get(fqid).expect("FQID {fqid} not found");
        self.symbols.get(id)
    }

    /// Get the Symbol mapped to the internal ID.
    pub fn find_by_id(&self, id: SymbolId) -> Option<&Symbol> {
        self.symbols.get(&id)
    }
}

/// Holds infromation about a single symbol in the source.
/// 
/// Q: should I store a path of scopes and a name and combine them into an FQID?
#[derive(Debug)]
pub struct Symbol {
  pub id: Option<SymbolId>,

  /// Path of the scopes leading to the symbol.
  pub namespace: Vec<String>,

  /// Name of the symbol.
  pub name: String, 
  pub kind: String,

  pub documentation: Option<Documentation>,
  pub source: String,

  pub parent: Option<SymbolId>,
  pub children: Vec<SymbolId>,
}

impl Symbol {
    pub fn new(name: String, kind: String, namespace: Vec<String>, source: &str, documentation: Option<Documentation>) -> Self {
      Self {
        id: None,
        namespace: namespace,
        name: name.to_string(),
        kind: kind,
        documentation: documentation,
        source: source.to_string(),
        parent: None,
        children: Vec::new(),
      }
    }

    pub fn fqid(&self) -> String {
        if self.namespace.is_empty() {
          return self.name.clone();
        }
        let path_str = self.namespace.join("::").to_string();
        let fqid = path_str + "::" + self.name.as_str();
        fqid
    }
}