use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct FileLines { pub file : Arc < SourceFile > , pub lines : Vec < LineInfo > , }