use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug)] struct KLevel { k : usize , node_count : usize , nodes : Vec < usize > , max_depth : usize , avg_depth : f64 , avg_complexity : f64 , critical_nodes : Vec < usize > , is_essential : bool , }
}