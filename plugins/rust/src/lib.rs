use common::{Documentation, LanguagePlugin, LanguageProcessor, Symbol};
use common::utility::{print_named_children};
use std::collections::HashSet;
use tree_sitter::Node;
use tree_sitter_rust;

pub mod plugin;
mod processor;
mod handlers;
