use tree_sitter::Parser;
use crate::Symbol;

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
    //fn extract_symbols(&self, source: &str) -> Vec<Symbol>;
    fn extract_symbols(&self, source: &str) -> Vec<Symbol> {
        //let mut stack: Vec<String> = Vec::new();
        let mut parser = Parser::new();
        parser.set_language(&self.language()).expect("Failed to set parser language.");

        let tree = parser.parse(source, None).expect("Failed to parse tree.");
        let symbols = Vec::new();
        let mut cursor = tree.walk();
        
        if cursor.goto_first_child() {
            loop {
                let node = cursor.node();
                let node_text = &source[node.byte_range()];

                println!("{:?}", node);
                println!("Child count: {}", node.child_count());
                println!("Named child count: {}", node.named_child_count());
                println!("Children:");
                for i in 0..node.child_count() {
                    println!("\t{:?}", node.child(i.try_into().unwrap()));
                }
                println!("{}", node_text);
                println!();
                
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }

        symbols
    }

    // Determines whether a node is a symbol or not.
    //fn is_symbol(&self, node: Node) -> bool;

    // Gets the name of the given node.
    //fn node_name(&self, node: Node, source: &str) -> String;

    // Determines whether a node is a comment or not.
    //fn is_comment(&self, node: Node) -> bool;

    // Determines where a comment should stick.
    //fn sticks_to(&self, node: Node, source: &str) -> StickLocation;
}
