use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < Crate > for CrateRootModuleId { fn from (krate : Crate) -> Self { CrateRootModuleId { krate } } }
}