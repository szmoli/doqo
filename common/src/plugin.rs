use std::collections::HashSet;
use crate::LanguageProcessor;

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
        self.plugins.push(support);
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

  fn id(&self) -> &'static str;

  fn symbol_kinds(&self) -> HashSet<&'static str>;

  /// File extensions associated with the language (eg. ".rs", ".py", ".ex")
  fn extensions(&self) -> HashSet<&'static str>;

  /// Processor for the specific language
  fn processor(&self) -> Box<dyn LanguageProcessor>;
}