use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: global_fn_name");
pub fn global_fn_name (base : Symbol) -> String { format ! ("__rust_{base}") }
}