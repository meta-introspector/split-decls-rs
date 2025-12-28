use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Serialize , Deserialize)] struct LayerOntology { layer : CompilerLayer , ngram_size : usize , top_patterns : Vec < LayerPattern > , total_patterns : usize , coverage_percentage : f64 , cross_layer_relations : Vec < CrossLayerRelation > , }