use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: is_likely_function");
fn is_likely_function (name : & str) -> bool { name . chars () . next () . map_or (false , | c | c . is_lowercase ()) }
}