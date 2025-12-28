use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct FileLines { pub file : Arc < SourceFile > , pub lines : Vec < LineInfo > , }
}