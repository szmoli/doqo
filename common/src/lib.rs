pub mod plugin;
pub mod processor;
pub mod utility;

mod source;
mod documentation;
mod session;
mod symbol;

pub use documentation::{Documentation, Metadata};
pub use plugin::{LanguagePlugin, LanguageRegistry};
pub use processor::LanguageProcessor;
pub use session::Session;
pub use symbol::{Symbol, SymbolId, SymbolTable};
