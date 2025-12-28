use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , PartialEq , Eq)] pub struct WorkspaceInfo { pub member_crates : Vec < String > , pub submodule_base_path_rel : std :: path :: PathBuf , }