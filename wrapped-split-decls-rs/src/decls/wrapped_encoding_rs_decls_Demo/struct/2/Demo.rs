use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [cfg (all (test , feature = "serde"))] # [derive (Serialize , Deserialize , Debug , PartialEq)] struct Demo { num : u32 , name : String , enc : & 'static Encoding , }