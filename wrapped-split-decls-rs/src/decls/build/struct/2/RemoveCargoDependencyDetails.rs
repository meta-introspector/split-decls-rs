use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Deserialize)] pub struct RemoveCargoDependencyDetails { pub target_file : PathBuf , pub package_name : String , }