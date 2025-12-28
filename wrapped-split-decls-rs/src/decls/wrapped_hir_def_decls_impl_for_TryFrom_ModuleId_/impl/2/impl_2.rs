use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl TryFrom < ModuleId > for CrateRootModuleId { type Error = () ; fn try_from (ModuleId { krate , block , local_id , } : ModuleId ,) -> Result < Self , Self :: Error > { if block . is_none () && local_id == DefMap :: ROOT { Ok (CrateRootModuleId { krate }) } else { Err (()) } } }
}