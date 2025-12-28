use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone , Serialize , Deserialize)] pub struct SerializableSpan { pub file_name : String , pub line_start : usize , pub col_start : usize , pub line_end : usize , pub col_end : usize , }
}