use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < CrateRootModuleId > for ModuleDefId { fn from (value : CrateRootModuleId) -> Self { ModuleDefId :: ModuleId (value . into ()) } }
}