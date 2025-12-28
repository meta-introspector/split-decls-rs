use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Serialize , Deserialize)] struct LayerAnalysis { layer : u8 , input_size : usize , output_size : usize , top_2grams : Vec < TwoGram > , preservation_proof : PreservationProof , }
}