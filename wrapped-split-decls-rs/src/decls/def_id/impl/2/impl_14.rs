use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl CrateNum { # [inline] pub fn new (x : usize) -> CrateNum { CrateNum :: from_usize (x) } # [inline] pub fn as_def_id (self) -> DefId { DefId { krate : self , index : CRATE_DEF_INDEX } } # [inline] pub fn as_mod_def_id (self) -> ModDefId { ModDefId :: new_unchecked (DefId { krate : self , index : CRATE_DEF_INDEX }) } }