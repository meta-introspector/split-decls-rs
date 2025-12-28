use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Represents a single patch entry for a crate."] # [cfg_attr (feature = "serde_enabled" , derive (Serialize , Deserialize))] # [derive (Debug , PartialEq , Eq , Hash)] pub struct PatchEntry { pub crate_name : String , pub path : PathBuf , }
}