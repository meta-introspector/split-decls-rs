use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Serialize , Deserialize)] pub struct SyscallTrait { pub name : String , pub category : SyscallCategory , pub methods : Vec < TraitMethod > , pub safety_level : SafetyLevel , }
}