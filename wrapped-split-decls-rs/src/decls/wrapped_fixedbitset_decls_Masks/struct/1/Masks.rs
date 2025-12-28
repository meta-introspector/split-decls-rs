use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct Masks { first_block : usize , first_mask : usize , last_block : usize , last_mask : usize , }
}