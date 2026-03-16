use crate::{LanguageRegistry, SymbolTable};

/// A session for multi-language projects
/// 
/// Q: A session fogja a saját fájljain végig hívni a LanguageProcessor extract_symbols függvényét, majd hozzáadni azokat a symbol_table-höz?
pub struct Session {
  /// Symbol table for the session.
  pub symbol_table: SymbolTable,

  /// Language plugins for the session.
  pub language_registry: LanguageRegistry,

  /// Source files of the project.
  pub source_files: Vec<String>,
}

impl Session {
  pub fn new() -> Self {
    Self {
      symbol_table: SymbolTable::new(),
      language_registry: LanguageRegistry::new(),
      source_files: Vec::new(),
    }
  }
}