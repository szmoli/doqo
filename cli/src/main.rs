use std::fs;

use common::LanguagePlugin;
use rust::{RustPlugin};

fn main() {
    let rust_plugin = RustPlugin;
    let source = fs::read_to_string("../input/in.rs").expect("Failed to read input");

    //println!("{}", source);

    rust_plugin.processor().extract_symbols(&source);
}