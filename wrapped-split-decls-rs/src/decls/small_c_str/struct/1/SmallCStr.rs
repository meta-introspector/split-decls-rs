use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Like SmallVec but for C strings."] # [derive (Clone)] pub struct SmallCStr { data : SmallVec < [u8 ; SIZE] > , }