use std::collections::{HashMap, HashSet};

pub struct Session {
  /// Symbol table for the session.
  pub symbol_table: SymbolTable,

  /// Language plugins for the session.
  pub language_registry: LanguageRegistry,
}

impl Session {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            language_registry: LanguageRegistry::new(),
        }
    }

    pub fn build_symbol_table(&mut self) {

    }
}

pub type SymbolId = usize;

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

    pub fn symbol_id(&self, fqid: &String) -> Option<&usize> {
        self.fqid_index.get(fqid)
    }

    pub fn register_symbol(&mut self, symbol: Symbol) {
        self.symbols.insert(self.current_id, symbol);
        self.current_id += 1;
    }

    pub fn find_by_fqid(&self, fqid: &String) -> Option<&Symbol> {
        let id = self.fqid_index.get(fqid).expect("FQID {fqid} not found");
        self.symbols.get(id)
    }

    pub fn find_by_id(&self, id: SymbolId) -> Option<&Symbol> {
        self.symbols.get(&id)
    }
}

pub struct Symbol {
  pub id: SymbolId,
  pub fqid: String,
  pub name: String, // can be derived from fqid I think

  pub parent: Option<SymbolId>,
  pub children: Vec<SymbolId>,

  // TODO: pub location: ?,
}

pub struct LanguageRegistry {
  plugins: Vec<Box<dyn LanguagePlugin>>,
}

impl LanguageRegistry {
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }

    pub fn register(&mut self, support: Box<dyn LanguagePlugin>) {
        self.plugins.push(support);
    }

    pub fn find_by_extension(&self, extension: &str) -> Option<&dyn LanguagePlugin> {
        self.plugins.iter()
            .find(|plugin| plugin.extensions().contains(extension))
            .map(|plugin| plugin.as_ref())
    }
}

pub trait LanguagePlugin {
  /*
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

// workspace leíró struct, pl: git full elérési útvonalakkal

pub trait LanguageProcessor {
  /*
    - kinyeri a symbolokat és a hozzá tapadó kommenteket
    - clean comments
    - scopeok kezelése (stackoverflows nestelt cucc, fqid -> symbol)
   */

  fn language(&self) -> tree_sitter::Language;
}
