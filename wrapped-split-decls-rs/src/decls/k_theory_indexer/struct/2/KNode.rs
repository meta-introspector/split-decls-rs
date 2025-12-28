use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Serialize , Deserialize)] pub struct KNode { pub name : String , pub level : u8 , pub index : i32 , pub complexity : f64 , pub depth : u32 , pub file_path : String , pub content : String , }