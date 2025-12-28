use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn calculate_similarity (content1 : & str , content2 : & str) -> f64 { let tokens1 : Vec < & str > = content1 . split_whitespace () . collect () ; let tokens2 : Vec < & str > = content2 . split_whitespace () . collect () ; let common_tokens = tokens1 . par_iter () . filter (| token | tokens2 . contains (token)) . count () ; let total_tokens = (tokens1 . len () + tokens2 . len ()) as f64 ; if total_tokens == 0.0 { 0.0 } else { (2.0 * common_tokens as f64) / total_tokens } }
}