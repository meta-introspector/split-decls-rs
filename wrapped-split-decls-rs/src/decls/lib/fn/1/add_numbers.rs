use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn add_numbers (a : i32 , b : i32) -> i32 { a + b }
}