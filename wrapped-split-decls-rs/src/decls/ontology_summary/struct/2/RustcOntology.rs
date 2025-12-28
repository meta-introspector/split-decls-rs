use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Serialize , Deserialize)] struct RustcOntology { ngram_size : usize , top_patterns : Vec < NGramPattern > , total_patterns : usize , coverage_percentage : f64 , }
}