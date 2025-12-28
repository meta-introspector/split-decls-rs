use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Default)] pub struct LineCountReport { pub input_lines : usize , pub output_lines : usize , pub skipped_items : Vec < SkippedItem > , pub processed_items : usize , pub error_items : Vec < ErrorItem > , }