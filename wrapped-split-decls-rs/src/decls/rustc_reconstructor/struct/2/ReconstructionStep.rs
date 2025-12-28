use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Serialize , Deserialize)] struct ReconstructionStep { layer : u8 , input_tokens : Vec < String > , output_patterns : Vec < String > , expansion_ratio : f64 , description : String , }
}