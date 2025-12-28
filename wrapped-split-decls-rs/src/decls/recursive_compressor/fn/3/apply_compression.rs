use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn apply_compression (text : & str , vocab : & HashMap < String , String >) -> String { let mut result = text . to_string () ; let mut sorted_patterns : Vec < _ > = vocab . iter () . collect () ; sorted_patterns . sort_by (| a , b | b . 1 . len () . cmp (& a . 1 . len ())) ; for (emoji , pattern) in sorted_patterns { result = result . replace (pattern , emoji) ; } result }