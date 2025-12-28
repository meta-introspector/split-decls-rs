use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Serialize)] struct NGramAnalysisSummary { ngram_size : usize , top_k : usize , total_patterns : usize , coverage_percentage : f64 , top_pattern : String , top_frequency : usize , }
}