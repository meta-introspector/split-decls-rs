use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Serialize , Debug , Clone)] pub struct DuplicateBlock { pub start_line_number : usize , pub end_line_number : usize , pub source_file : String , }
}