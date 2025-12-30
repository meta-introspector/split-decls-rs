use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: add_numbers");
pub fn add_numbers (a : i32 , b : i32) -> i32 { a + b }
}