use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Default , Serialize , Deserialize)] pub struct PatternStats { pub count : u64 , pub ast_nodes : Vec < String > , pub complexity_score : f64 , }
}