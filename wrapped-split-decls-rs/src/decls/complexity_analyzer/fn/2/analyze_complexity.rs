use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn analyze_complexity (item : & Item , file_path : & str) -> Option < ComplexityReport > { match item { Item :: Struct (s) => analyze_struct_complexity (s , file_path) , Item :: Enum (e) => analyze_enum_complexity (e , file_path) , _ => None , } }
}