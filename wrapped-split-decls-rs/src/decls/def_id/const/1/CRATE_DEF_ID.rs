use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
pub const CRATE_DEF_ID : LocalDefId = LocalDefId { local_def_index : CRATE_DEF_INDEX } ;
}