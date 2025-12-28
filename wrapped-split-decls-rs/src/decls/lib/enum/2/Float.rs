use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Floating-point types."] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] # [cfg_attr (feature = "nightly" , derive (HashStable_Generic))] pub enum Float { F16 , F32 , F64 , F128 , }