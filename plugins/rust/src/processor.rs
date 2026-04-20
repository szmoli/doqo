use std::collections::{HashMap, HashSet};

use common::{
    LanguageProcessor, processor::{NodeHandler},
};

use crate::handlers::{
    handle_const_item, handle_enum_item, handle_enum_variant, handle_field_declaration, handle_function_item, handle_line_comment, handle_macro_definition, handle_mod_item, handle_source_file, handle_struct_item, handle_trait_item, handle_type_item
};

pub struct RustProcessor {
    handlers: HashMap<&'static str, NodeHandler>,
    comment_clearers: HashSet<&'static str>,
}

impl RustProcessor {
    pub fn new() -> Self {
        let mut handlers: HashMap<&'static str, NodeHandler> = HashMap::new();

        handlers.insert("struct_item", handle_struct_item);
        handlers.insert("enum_item", handle_enum_item);
        handlers.insert("enum_variant", handle_enum_variant);
        handlers.insert("function_item", handle_function_item);
        handlers.insert("trait_item", handle_trait_item);
        handlers.insert("mod_item", handle_mod_item);
        handlers.insert("line_comment", handle_line_comment);
        handlers.insert("const_item", handle_const_item);
        handlers.insert("field_declaration", handle_field_declaration);
        handlers.insert("type_item", handle_type_item);
        handlers.insert("macro_definition", handle_macro_definition);
        handlers.insert("source_file", handle_source_file);

        Self {
            handlers: handlers,
            comment_clearers: HashSet::from(["use_declaration", "let_declaration", "macro_invocation"]),
        }
    }
}

impl LanguageProcessor for RustProcessor {
    fn language(&self) -> tree_sitter::Language {
        tree_sitter_rust::LANGUAGE.into()
    }

    /// Returns true if stack was pushed. Returns false otherwise.
    fn handle_node(
        &self,
        node: tree_sitter::Node,
        source: &str,
        context: &mut common::processor::ProcessingContext,
    ) -> bool {
        if let Some(handler) = self.handlers.get(node.kind()) {
            handler(node, source, context)
        } else {
            if self.comment_clearers.contains(node.kind()) {
                let _comment = context.take_comments(); // clear comments
            }

            false
        }
    }
}
