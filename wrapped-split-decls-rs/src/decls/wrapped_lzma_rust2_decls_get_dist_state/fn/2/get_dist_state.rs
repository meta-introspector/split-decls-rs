use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub (crate) fn get_dist_state (len : u32) -> u32 { (if (len as usize) < DIST_STATES + MATCH_LEN_MIN { len as usize - MATCH_LEN_MIN } else { DIST_STATES - 1 }) as u32 }
}