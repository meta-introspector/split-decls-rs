use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Serialize , Deserialize)] pub struct SourceArrow { pub file_path : String , pub line : usize , pub column : usize , pub angle : f64 , pub length : f64 , }