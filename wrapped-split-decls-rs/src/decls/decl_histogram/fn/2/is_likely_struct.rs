use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: is_likely_struct");
fn is_likely_struct (name : & str) -> bool { name . chars () . next () . map_or (false , | c | c . is_uppercase ()) }
}