#[derive(Debug)]
pub struct Documentation {
  pub content: String,
  pub metadata: Vec<Metadata>,
}

impl Documentation {
  pub fn new(content: String) -> Self {
    Self {
      content: content,
      metadata: Vec::new(),
    }
  }
}

#[derive(Debug)]
pub struct Metadata;

// Possible locations to stick a comment to.
/*
pub enum StickLocation {
    ParentSymbol,   // Sticks to the parent
    NextSymbol,     // Sticks to the following symbol
    None            // Is ignored
}
*/
