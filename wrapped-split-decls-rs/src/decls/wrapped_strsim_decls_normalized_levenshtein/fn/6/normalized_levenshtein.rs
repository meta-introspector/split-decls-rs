use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: normalized_levenshtein");
# [doc = " Calculates a normalized score of the Levenshtein algorithm between 0.0 and"] # [doc = " 1.0 (inclusive), where 1.0 means the strings are the same."] # [doc = ""] # [doc = " ```"] # [doc = " use strsim::normalized_levenshtein;"] # [doc = ""] # [doc = " assert!((normalized_levenshtein(\"kitten\", \"sitting\") - 0.57142).abs() < 0.00001);"] # [doc = " assert!((normalized_levenshtein(\"\", \"\") - 1.0).abs() < 0.00001);"] # [doc = " assert!(normalized_levenshtein(\"\", \"second\").abs() < 0.00001);"] # [doc = " assert!(normalized_levenshtein(\"first\", \"\").abs() < 0.00001);"] # [doc = " assert!((normalized_levenshtein(\"string\", \"string\") - 1.0).abs() < 0.00001);"] # [doc = " ```"] pub fn normalized_levenshtein (a : & str , b : & str) -> f64 { if a . is_empty () && b . is_empty () { return 1.0 ; } 1.0 - (levenshtein (a , b) as f64) / (a . chars () . count () . max (b . chars () . count ()) as f64) }
}