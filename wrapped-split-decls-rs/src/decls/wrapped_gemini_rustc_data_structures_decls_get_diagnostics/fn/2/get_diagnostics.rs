use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: get_diagnostics");
pub fn get_diagnostics () -> Vec < SerializableDiagnostic > { COLLECTED_DIAGNOSTICS . with (| diagnostics | diagnostics . read () . clone ()) }
}