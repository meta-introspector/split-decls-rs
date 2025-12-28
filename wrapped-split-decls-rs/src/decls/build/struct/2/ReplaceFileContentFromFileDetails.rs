use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Deserialize)] pub struct ReplaceFileContentFromFileDetails { pub target_file : PathBuf , pub source_file : PathBuf , }
}