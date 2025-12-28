use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A `u128` but encoded with a fixed size; for hashes this encoding is more compact than `u128`."] # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Default)] pub struct Hash128 { inner : u128 , }