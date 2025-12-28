use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn extract_context (symbol : & str , _pos : usize) -> String { if let Some (first_part) = symbol . split ("::") . next () { first_part . to_string () } else { "unknown" . to_string () } }
}