use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Deserialize)] pub struct AddFunctionDetails { pub target_file : PathBuf , pub function_code : String , }
}