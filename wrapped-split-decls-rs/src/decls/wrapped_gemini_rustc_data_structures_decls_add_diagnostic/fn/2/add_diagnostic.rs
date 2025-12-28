use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn add_diagnostic (diag : SerializableDiagnostic) { COLLECTED_DIAGNOSTICS . with (| diagnostics | { diagnostics . write () . push (diag) ; }) ; }
}