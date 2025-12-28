use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: diagnostics_registry");
pub fn diagnostics_registry () -> Registry { Registry :: new (rustc_errors :: codes :: DIAGNOSTICS) }
}