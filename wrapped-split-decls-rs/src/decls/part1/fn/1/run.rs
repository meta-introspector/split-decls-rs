use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn run () { println ! ("cargo:warning=Hello from target_crate's build.rs part1!") ; }
}