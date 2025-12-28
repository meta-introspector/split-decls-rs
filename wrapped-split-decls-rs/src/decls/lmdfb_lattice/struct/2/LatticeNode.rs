use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Serialize , Deserialize)] struct LatticeNode { symbol : String , emoji : String , level : u32 , weight : f64 , layer : String , dependencies : Vec < String > , complexity_score : f64 , }