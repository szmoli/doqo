use std::{fs::{self, File}, io::{BufWriter, Write}, path::{Path, PathBuf}};

use common::{LanguagePlugin, SymbolTable};
use rust::plugin::RustPlugin;

fn main() {
    let mut symbol_table = SymbolTable::new();
    let rust_plugin = RustPlugin;
    let path = PathBuf::from("../input/in.rs");

    //println!("{}", source);

    rust_plugin.processor().process(&path, &mut symbol_table);

    let mut sorted_symbols: Vec<_> = symbol_table.symbols.iter().collect();
    sorted_symbols.sort_by_key(|&(id, _)| id);

    println!("Finished processing symbols:");
    for symbol in sorted_symbols {
      println!("{:?}", symbol);
      println!();
    }

    let json = symbol_table.json();
    let path = Path::new("json/symbol_table.json");
    let file = File::create(path).expect(format!("Unable to create file at {}", path.display()).as_str());
    let mut writer = BufWriter::new(file);
    writer.write_all(json.as_bytes()).expect("Unable to write data");
    writer.flush().expect("Unable to flush buffer");
    
    println!("JSON:\n{}", json)

    
}