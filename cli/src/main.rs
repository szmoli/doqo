use std::fs;

use common::{LanguagePlugin, SymbolTable};
use rust::{RustPlugin};

fn main() {
    let mut symbol_table = SymbolTable::new();
    let rust_plugin = RustPlugin;
    let source = fs::read_to_string("../input/in.rs").expect("Failed to read input");

    //println!("{}", source);

    rust_plugin.processor().process(&source, &mut symbol_table);

    let mut sorted_symbols: Vec<_> = symbol_table.symbols.iter().collect();
    sorted_symbols.sort_by_key(|&(id, _)| id);

    println!("Finished processing symbols:");
    for symbol in sorted_symbols {
      println!("{:?}", symbol);
      println!();
    }
}