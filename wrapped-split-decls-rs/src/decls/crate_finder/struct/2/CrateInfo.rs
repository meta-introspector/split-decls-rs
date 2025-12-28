use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Deserialize , Serialize , Clone)] pub struct CrateInfo { pub name : String , pub original_path : PathBuf , }
}