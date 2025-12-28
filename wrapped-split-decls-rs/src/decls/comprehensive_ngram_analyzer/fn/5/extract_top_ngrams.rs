use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn extract_top_ngrams (patterns : & [serde_json :: Value] , n : usize) -> Vec < NGram > { let mut ngram_counts = HashMap :: new () ; for pattern in patterns { let pattern_str = pattern . as_str () . unwrap_or ("") ; let tokens = tokenize_pattern (pattern_str) ; for window in tokens . windows (n) { if window . len () == n { let ngram = window . to_vec () ; * ngram_counts . entry (ngram) . or_insert (0) += 1 ; } } } let mut sorted_ngrams : Vec < _ > = ngram_counts . iter () . collect () ; sorted_ngrams . sort_by (| a , b | b . 1 . cmp (a . 1)) ; sorted_ngrams . iter () . take (10) . map (| (tokens , count) | NGram { tokens : tokens . to_vec () , count : * * count , pattern_type : classify_ngram_pattern (tokens , n) , }) . collect () }
}