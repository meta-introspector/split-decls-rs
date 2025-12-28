use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A `ModuleId` that is always a crate's root module."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct CrateRootModuleId { krate : Crate , }
}