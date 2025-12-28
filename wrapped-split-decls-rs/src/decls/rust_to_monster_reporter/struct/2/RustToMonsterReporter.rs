use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Generate the epic Rust to Monster Group transformation report"] pub struct RustToMonsterReporter { pub compressor : MonsterCompressor , pub total_signatures : usize , pub total_declarations : u64 , pub monster_coverage : f64 , }
}