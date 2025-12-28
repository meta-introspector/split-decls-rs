use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn analyze_common_forms (patterns : & [serde_json :: Value] , label : & str) -> Vec < (String , usize) > { let mut pattern_counts = HashMap :: new () ; for pattern in patterns { let pattern_str = pattern . as_str () . unwrap_or ("") ; let common_form = extract_common_form (pattern_str) ; * pattern_counts . entry (common_form) . or_insert (0) += 1 ; } let mut sorted_patterns : Vec < _ > = pattern_counts . into_iter () . collect () ; sorted_patterns . sort_by (| a , b | b . 1 . cmp (& a . 1)) ; sorted_patterns }
}