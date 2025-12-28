use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug)] pub struct ComplexityResult { pub function_name : String , pub complexity : f64 , pub file_path : String , pub frequency : usize , }