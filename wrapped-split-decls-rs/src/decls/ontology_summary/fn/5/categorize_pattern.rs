use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn categorize_pattern (pattern : & str) -> String { if pattern . contains ("use") && pattern . contains ("std") { "standard_library_imports" . to_string () } else if pattern . contains ("serde") { "serialization_framework" . to_string () } else if pattern . contains ("rustc") || pattern . contains ("hir") || pattern . contains ("mir") { "compiler_internals" . to_string () } else if pattern . contains ("fn") || pattern . contains ("impl") { "function_definitions" . to_string () } else if pattern . contains ("struct") || pattern . contains ("enum") { "data_structures" . to_string () } else if pattern . contains ("error") || pattern . contains ("Result") { "error_handling" . to_string () } else if pattern . contains ("test") { "testing_framework" . to_string () } else if pattern . contains ("macro") { "macro_system" . to_string () } else { "general_patterns" . to_string () } }
}