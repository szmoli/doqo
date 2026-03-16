pub struct Documentation {
  pub content: String,
  pub metadata: Vec<Metadata>,
}

pub struct Metadata;

// Possible locations to stick a comment to.
/*
pub enum StickLocation {
    ParentSymbol,   // Sticks to the parent
    NextSymbol,     // Sticks to the following symbol
    None            // Is ignored
}
*/
