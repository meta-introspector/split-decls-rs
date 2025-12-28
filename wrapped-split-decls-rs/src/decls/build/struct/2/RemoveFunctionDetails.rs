use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Deserialize)] pub struct RemoveFunctionDetails { pub target_file : PathBuf , pub function_name : String , }
}