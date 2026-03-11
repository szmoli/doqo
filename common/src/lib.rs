use std::collections::{HashMap, HashSet};

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

/// The internal ID of a symbol.
/// 
/// Q: change to some kind of hash instead of usize?
pub type SymbolId = usize;

/// The symbol table. Maps IDs or FQIDs to Symbols.
/// The current_id starts from 0 and is incremented every time a new symbol is registered. 
pub struct SymbolTable {
  symbols: HashMap<SymbolId, Symbol>,
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
    pub fn register_symbol(&mut self, symbol: Symbol) {
        self.symbols.insert(self.current_id, symbol);
        self.current_id += 1;
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
pub struct Symbol {
  pub id: SymbolId,

  /// Path of the scopes leading to the symbol.
  pub path: Vec<String>,

  /// Name of the symbol.
  pub name: String, 

  pub parent: Option<SymbolId>,
  pub children: Vec<SymbolId>,
}

impl Symbol {
    pub fn fqid(&self) -> String {
        let path_str = self.path.join("::").to_string();
        let fqid = path_str + "::" + self.name.as_str();
        fqid
    }
}

/// Holds the known language plugins.
pub struct LanguageRegistry {
  plugins: Vec<Box<dyn LanguagePlugin>>,
}

impl LanguageRegistry {
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }

    /// Register a language plugin.
    pub fn register(&mut self, support: Box<dyn LanguagePlugin>) {
        self.plugins.push(support);()
    }

    /// Get the language plugin mapped to the file extension.
    pub fn find_by_extension(&self, extension: &str) -> Option<&dyn LanguagePlugin> {
        self.plugins.iter()
            .find(|plugin| plugin.extensions().contains(extension))
            .map(|plugin| plugin.as_ref())
    }
}

/// Describes a language.
pub trait LanguagePlugin {
  /*
    Konzi jegyzetek:
    - kiterjesztések
    - symbol nevek/típusok (treesitter entity - role)
    - nyelv neve
    - language processor (másik trait)
   */

  /// Human readable name of the language (eg. "Rust", "Python", "Elixir")
  fn name(&self) -> &'static str;

  /// File extensions associated with the language (eg. ".rs", ".py", ".ex")
  fn extensions(&self) -> HashSet<&'static str>;

  /// Processor for the specific language
  fn processor(&self) -> Box<dyn LanguageProcessor>;
}

/// Processes a specific language into symbols.
pub trait LanguageProcessor {
    /*
    Konzi jegyzetek:
    - workspace leíró struct, pl: git full elérési útvonalakkal
    - kinyeri a symbolokat és a hozzá tapadó kommenteket
    - clean comments
    - scopeok kezelése (stackoverflows nestelt cucc, fqid -> symbol)
    */

    /// Get the Tree Sitter grammar for the language.
    fn language(&self) -> tree_sitter::Language;

    /// Extract the symbols from a source string.
    fn extract_symbols(&self, source: &str) -> Vec<Symbol>;
}
