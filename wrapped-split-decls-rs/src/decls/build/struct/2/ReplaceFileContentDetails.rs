use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Deserialize)] pub struct ReplaceFileContentDetails { pub target_file : PathBuf , pub new_content : String , }
}