use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub (crate) struct LiteralCoder { lc : u32 , literal_pos_mask : u32 , }
}