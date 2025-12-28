use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Serialize , Deserialize)] struct LayerVocab { layer : u8 , vocab : HashMap < String , String > , compression_ratio : f64 , parent_layer : Option < u8 > , }