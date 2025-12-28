use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < LocalDefId > for DefId { fn from (local : LocalDefId) -> DefId { local . to_def_id () } }