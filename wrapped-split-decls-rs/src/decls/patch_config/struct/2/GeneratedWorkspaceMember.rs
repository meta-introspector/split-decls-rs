use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Deserialize , Serialize)] pub struct GeneratedWorkspaceMember { pub name : String , pub path : PathBuf , }
}