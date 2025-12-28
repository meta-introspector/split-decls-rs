use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn calculate_level (symbol : & str) -> u32 { let parts : Vec < & str > = symbol . split ("::") . collect () ; let base_level = parts . len () as u32 ; if symbol . contains ("rustc_") { base_level + 3 } else if symbol . contains ("std::") || symbol . contains ("core::") { base_level + 2 } else if symbol . contains ("alloc") || symbol . contains ("hash") { base_level + 1 } else { base_level } }
}