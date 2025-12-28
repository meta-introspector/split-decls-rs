use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn extract_macro_name (line : & str) -> Option < String > { if let Some (start) = line . find ("sys:macro") { if let Some (name_start) = line [start ..] . find ('"') { if let Some (name_end) = line [start + name_start + 1 ..] . find ('"') { return Some (line [start + name_start + 1 .. start + name_start + 1 + name_end] . to_string ()) ; } } } None }