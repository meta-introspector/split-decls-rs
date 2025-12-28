use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn get_diagnostics () -> Vec < SerializableDiagnostic > { COLLECTED_DIAGNOSTICS . with (| diagnostics | diagnostics . read () . clone ()) }
}