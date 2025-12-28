use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Serialize , Deserialize)] struct RecursiveCompression { layers : Vec < LayerVocab > , total_compression : f64 , original_patterns : usize , final_vocab_size : usize , }