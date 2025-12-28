use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Deserialize)] pub struct ReplaceExpressionDetails { pub target_file : PathBuf , pub function_name : String , pub old_code_snippet : String , pub new_code_snippet : String , }