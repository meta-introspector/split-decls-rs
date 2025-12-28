use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct CachedModuleCodegen { pub name : String , pub source : WorkProduct , }
}