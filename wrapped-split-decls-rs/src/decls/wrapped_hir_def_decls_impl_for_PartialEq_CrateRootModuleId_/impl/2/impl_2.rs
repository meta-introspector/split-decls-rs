use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl PartialEq < CrateRootModuleId > for ModuleId { fn eq (& self , other : & CrateRootModuleId) -> bool { other == self } }
}