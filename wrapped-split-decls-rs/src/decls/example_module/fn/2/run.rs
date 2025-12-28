use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: run");
pub fn run () { println ! ("cargo:warning=Running build logic from example_module!") ; }
}