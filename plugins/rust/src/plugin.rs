use std::collections::HashSet;

use common::{LanguagePlugin, LanguageProcessor};

use crate::processor::RustProcessor;

pub struct RustPlugin;

impl LanguagePlugin for RustPlugin {
    fn name(&self) -> &'static str {
        "Rust"
    }

    fn extensions(&self) -> HashSet<&'static str> {
        HashSet::from(["rs"])
    }

    fn processor(&self) -> Box<dyn LanguageProcessor> {
        Box::new(RustProcessor::new())
    }

    fn id(&self) -> &'static str {
        "rust"
    }

    fn symbol_kinds(&self) -> HashSet<&'static str> {
        // TODO
        HashSet::from(["struct_item"])
        //kinds.iter().map)
    }
}