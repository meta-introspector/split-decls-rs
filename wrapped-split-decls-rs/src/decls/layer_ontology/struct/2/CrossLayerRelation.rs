use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Serialize , Deserialize)] struct CrossLayerRelation { pattern : String , source_layer : CompilerLayer , target_layer : CompilerLayer , relation_strength : f64 , shared_nodes : usize , }
}