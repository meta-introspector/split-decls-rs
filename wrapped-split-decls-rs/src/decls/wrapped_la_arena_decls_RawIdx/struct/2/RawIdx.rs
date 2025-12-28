use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " The raw index of a value in an arena."] # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct RawIdx (u32) ;