use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: default_fn_name");
pub fn default_fn_name (base : Symbol) -> String { format ! ("__rdl_{base}") }
}