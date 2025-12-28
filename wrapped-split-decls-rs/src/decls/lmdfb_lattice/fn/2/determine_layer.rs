use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn determine_layer (symbol : & str , level : u32) -> String { match level { 0 ..= 2 => "foundation" . to_string () , 3 ..= 4 => "system" . to_string () , 5 ..= 6 => "compiler" . to_string () , 7 ..= 8 => "frontend" . to_string () , _ => "application" . to_string () , } }
}