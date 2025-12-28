use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait HasModule { # [doc = " Returns the enclosing module this thing is defined within."] fn module (& self , db : & dyn DefDatabase) -> ModuleId ; # [doc = " Returns the crate this thing is defined within."] # [inline] # [doc (alias = "crate")] fn krate (& self , db : & dyn DefDatabase) -> Crate { self . module (db) . krate } }