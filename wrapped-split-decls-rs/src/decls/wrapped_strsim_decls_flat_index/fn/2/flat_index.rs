use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: flat_index");
fn flat_index (i : usize , j : usize , width : usize) -> usize { j * width + i }
}