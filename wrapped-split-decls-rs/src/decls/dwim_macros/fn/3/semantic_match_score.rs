use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn semantic_match_score (intent : & DwimIntent , macro_def : & MacroDefinition) -> f64 { if intent . keywords . contains (& "bootstrap" . to_string ()) && macro_def . name . contains ("bootstrap") { 0.9 } else if intent . keywords . contains (& "build" . to_string ()) && macro_def . name . contains ("build") { 0.8 } else { 0.0 } }
}