use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Records information about a string"] # [derive (Debug , Clone)] pub struct StringRecord { pub content : String , pub file : String , pub line : usize , pub column : usize , pub byte_offset : usize , pub byte_length : usize , }