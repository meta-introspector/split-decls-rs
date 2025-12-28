use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn extract_2grams_from_patterns (patterns : & [serde_json :: Value]) -> HashMap < (String , String) , usize > { let mut twograms = HashMap :: new () ; for pattern in patterns { let pattern_str = pattern . as_str () . unwrap_or ("") ; let tokens = tokenize_pattern (pattern_str) ; for window in tokens . windows (2) { if window . len () == 2 { let pair = (window [0] . clone () , window [1] . clone ()) ; * twograms . entry (pair) . or_insert (0) += 1 ; } } } twograms }