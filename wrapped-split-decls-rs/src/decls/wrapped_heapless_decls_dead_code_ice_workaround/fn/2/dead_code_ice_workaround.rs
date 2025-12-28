use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: dead_code_ice_workaround");
# [expect (dead_code)] fn dead_code_ice_workaround () { }
}