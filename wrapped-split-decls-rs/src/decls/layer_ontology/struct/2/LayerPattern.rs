use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone , Serialize , Deserialize)] struct LayerPattern { pattern : String , frequency : usize , nodes : Vec < String > , emoji : String , semantic_label : String , layer : CompilerLayer , }
}