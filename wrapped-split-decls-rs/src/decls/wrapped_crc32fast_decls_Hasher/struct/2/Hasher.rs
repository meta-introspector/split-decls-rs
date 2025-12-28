use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone)] # [doc = " Represents an in-progress CRC32 computation."] pub struct Hasher { amount : u64 , state : State , }
}