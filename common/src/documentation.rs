#[derive(Debug)]
pub struct Documentation {
  raw: String,
  cleaned: Option<String>,
  //range: std::ops::Range<usize>,
  metadata: Vec<Metadata>,
}

impl Documentation {
  pub fn new(raw: String) -> Self {
    Self {
      raw: raw,
      cleaned: None,
      metadata: Vec::new(),
    }
  }

  /// Returns the raw documentation.
  pub fn raw(&self) -> String {
    self.raw.clone()
  }

  /// Returns the cleand documentation.
  // TODO: do lazy evaluation without a &mut self.
  pub fn cleaned(&mut self) -> String {
    if let Some(cleaned) = self.cleaned.clone() {
      cleaned
    }
    else {
      // TODO: clean the raw
      let cleaned = self.raw.clone();
      cleaned
    }
  }

  /// Append to the documentation.
  /// Side effect: clears cleaned.
  pub fn append(&mut self, raw: String) {
    self.raw.push_str(&raw);
    self.cleaned = None;
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
