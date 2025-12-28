use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug)] pub struct ModuleNotFoundReport { pub module_name : String , pub error : String , }
}