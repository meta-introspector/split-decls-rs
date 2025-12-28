use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone)] pub struct ModuleNotFoundReport { pub crate_name : String , pub module_name : String , pub error_message : String , pub generated_file_path : PathBuf , }