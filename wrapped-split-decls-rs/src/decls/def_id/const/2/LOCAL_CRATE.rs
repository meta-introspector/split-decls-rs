use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
# [doc = " Item definitions in the currently-compiled crate would have the `CrateNum`"] # [doc = " `LOCAL_CRATE` in their `DefId`."] pub const LOCAL_CRATE : CrateNum = CrateNum :: ZERO ;
}