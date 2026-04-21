use std::{collections::HashMap, path::{PathBuf}};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub type FileId = usize;

#[derive(Debug, Serialize, Deserialize, TS)]
pub struct SourceFile {
    pub path: PathBuf,
    pub content: String,
}