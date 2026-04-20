use serde::{Serialize, Deserialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Documentation {
  pub source: String,
  comments: Vec<String>,
  //range: std::ops::Range<usize>,
  //metadata: Vec<Metadata>,
}

impl Documentation {
  pub fn new(source: &str, comments: &[String]) -> Self {
    Self {
      source: String::from(source),
      comments: Vec::from(comments)
    }
  }

  /* 
  pub fn source(&self) -> &str {
    self.source.as_str()
  }
  */

  pub fn comments(&self) -> String {
    self.comments.join("\n")
  }

  pub fn append(&mut self, comment: &str) {
    self.comments.push(String::from(comment));
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
