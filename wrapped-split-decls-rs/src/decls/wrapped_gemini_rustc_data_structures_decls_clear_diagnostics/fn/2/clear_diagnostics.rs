use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: clear_diagnostics");
pub fn clear_diagnostics () { COLLECTED_DIAGNOSTICS . with (| diagnostics | { diagnostics . write () . clear () ; }) ; }
}