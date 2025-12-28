use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone)] struct ComplexityReport { name : String , complexity : u8 , field_count : usize , variant_count : usize , nested_depth : u8 , file_path : String , }
}