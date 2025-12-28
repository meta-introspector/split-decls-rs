use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Serialize , Deserialize)] struct LayerNGramAnalysis { layer : u8 , input_size : usize , output_size : usize , ngram_2 : Vec < NGram > , ngram_3 : Vec < NGram > , ngram_5 : Vec < NGram > , ngram_7 : Vec < NGram > , }
}