use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: run");
pub fn run () { println ! ("cargo:warning=Running OS-specific build logic wrapper.") ; os_impl :: run () ; }
}