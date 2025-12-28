use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: reverse_layer");
fn reverse_layer (vocab : & HashMap < String , String > , input_tokens : & [String]) -> Vec < String > { let mut expanded = Vec :: new () ; for token in input_tokens { if let Some (pattern) = vocab . get (token) { expanded . push (pattern . clone ()) ; } else { expanded . push (token . clone ()) ; } } expanded }
}