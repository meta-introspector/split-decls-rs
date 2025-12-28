use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Default)] struct CrateMetrics { name : String , functions : usize , structs : usize , enums : usize , macros : usize , loc : usize , }
}