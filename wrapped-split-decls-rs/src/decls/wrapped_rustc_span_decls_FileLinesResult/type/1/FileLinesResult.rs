use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type FileLinesResult = Result < FileLines , SpanLinesError > ;
}