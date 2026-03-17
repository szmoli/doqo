pub fn print_named_children(node: tree_sitter::Node, source: &str) {
    let count = node.named_child_count() as u32;
    println!("Node '{}' has {} named children:", node.kind(), count);

    for i in 0..count {
        if let Some(child) = node.named_child(i) {
            let text = &source[child.byte_range()];
            println!(
                "  [{}] Kind: {:15} | Text: \"{}\"",
                i,
                child.kind(),
                text.trim()
            );
        }
    }
}