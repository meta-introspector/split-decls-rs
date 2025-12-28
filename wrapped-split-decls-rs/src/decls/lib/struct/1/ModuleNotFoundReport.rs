use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone)] pub struct ModuleNotFoundReport { pub module_name : String , pub error : String , }